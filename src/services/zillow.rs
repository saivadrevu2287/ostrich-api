use crate::{
    config::Config,
    error::{OstrichError, OstrichErrorType},
    models::{emailer::Emailer, listing_data::NewListingData},
    db_conn::DbConn
};
use tokio_stream::{self as stream, StreamExt};
use reqwest::Error;
use serde_derive::Deserialize;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use urlencoding::encode;

#[derive(Deserialize)]
pub struct ZillowSearchParameters {
    pub search_param: String,
    pub max_price: Option<f64>,
    pub min_price: Option<f64>,
    pub no_bedrooms: Option<i32>,
}

impl From<Error> for OstrichError {
    fn from(e: Error) -> OstrichError {
        OstrichError::new(
            format!("Failed to fetch data {:?}", e),
            OstrichErrorType::ApiError,
        )
    }
}

#[derive(Deserialize, Debug)]
pub struct ListingResultsSubType {
    pub is_FSBA: Option<bool>,
    pub is_openHouse: Option<bool>,
    pub is_newHome: Option<bool>,
    pub is_bankOwned: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct ListingResultsProp {
    pub dateSold: Option<String>,
    pub propertyType: Option<String>,
    pub lotAreaValue: Option<f64>,
    pub address: Option<String>,
    pub daysOnZillow: Option<f64>,
    pub price: Option<f64>,
    pub unit: Option<String>,
    pub listingDateTime: Option<String>,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub contingentListingType: Option<String>,
    pub listingStatus: Option<String>,
    pub zpid: Option<String>,
    #[serde(skip)]
    pub listingSubType: Option<ListingResultsSubType>,
    pub imgSrc: Option<String>,
    pub livingArea: Option<f64>,
    pub bathrooms: Option<f64>,
    pub lotAreaUnit: Option<String>,
    pub country: Option<String>,
    pub currency: Option<String>,
    pub bedrooms: Option<f64>,
    pub hasImage: Option<bool>,
    pub newConstructionType: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ZillowListingsSearchRoot {
    pub props: Vec<ListingResultsProp>,
    pub resultsPerPage: Option<i64>,
    pub totalResultCount: Option<i64>,
    pub totalPages: Option<i64>,
}

pub async fn get_zillow_listing_results(
    config: Arc<Config>,
    reqwest_client: Arc<reqwest::Client>,
    api_url: String,
) -> Result<ZillowListingsSearchRoot, OstrichError> {
    log::info!("Getting listings at {}", api_url);

    let listing_data = reqwest_client
        .get(api_url)
        .header("X-RapidAPI-Host", config.zillow_api.api_host.clone())
        .header("X-RapidAPI-Key", config.zillow_api.api_key.clone())
        .send()
        .await?
        .json::<ZillowListingsSearchRoot>()
        .await
        .map_err(|e| {
            OstrichError::new(
                format!(
                    "Search Parameters did not result in a valid response {:?}",
                    e
                ),
                OstrichErrorType::ListingResultError,
            )
        })?;

    Ok(listing_data)
}

pub fn get_zillow_listing_url_from_params(
    config: Arc<Config>,
    zillow_search_params: &ZillowSearchParameters,
    days_on_market: Option<i32>,
) -> String {
    let mut api_url = format!(
        //?location=northampton%20county&home_type=Houses&minPrice=100000&maxPrice=200000&daysOn=1
        "https://{}/propertyExtendedSearch?location={}",
        config.zillow_api.api_host,
        encode(&zillow_search_params.search_param)
    );

    if zillow_search_params.max_price.is_some() {
        api_url = format!(
            "{}&maxPrice={}",
            api_url,
            zillow_search_params.max_price.unwrap()
        );
    }

    if zillow_search_params.min_price.is_some() {
        api_url = format!(
            "{}&minPrice={}",
            api_url,
            zillow_search_params.min_price.unwrap()
        );
    }

    if days_on_market.is_some() {
        api_url = format!("{}&daysOn={}", api_url, days_on_market.unwrap());
    }

    api_url
}

#[derive(Deserialize, Debug)]
pub struct ZillowPropertySearchRoot {
    pub buildingPermits: Option<String>,
    pub propertyTaxRate: Option<f64>,
    pub longitude: Option<f64>,
    pub countyFIPS: Option<String>,
    pub cityId: Option<i64>,
    pub timeOnZillow: Option<String>,
    pub url: Option<String>,
    pub zestimate: Option<i64>,
    pub imgSrc: Option<String>,
    pub zpid: Option<i64>,
    pub zipcode: Option<String>,
    pub livingAreaValue: Option<i64>,
    pub zestimateLowPercent: Option<String>,
    pub isListedByOwner: Option<bool>,
    pub propertyTypeDimension: Option<String>,
    pub streetAddress: Option<String>,
    pub county: Option<String>,
    pub stateId: Option<i64>,
    pub countyId: Option<i64>,
    pub timeZone: Option<String>,
    pub homeType: Option<String>,
    pub livingAreaUnits: Option<String>,
    pub comingSoonOnMarketDate: Option<String>,
    pub livingArea: Option<i64>,
    pub bathrooms: Option<i64>,
    pub annualHomeownersInsurance: Option<i64>,
    pub state: Option<String>,
    pub rentZestimate: Option<f64>,
    pub building: Option<String>,
    pub brokerId: Option<String>,
    pub yearBuilt: Option<i64>,
    pub brokerageName: Option<String>,
    pub dateSold: Option<String>,
    pub price: Option<f64>,
    pub pageViewCount: Option<i64>,
    pub description: Option<String>,
    pub mortgageRates: Option<MortgageRates>,
    pub homeStatus: Option<String>,
    pub homeFacts: Option<String>,
    pub latitude: Option<f64>,
    pub datePosted: Option<String>,
    pub bedrooms: Option<i64>,
    pub monthlyHoaFee: Option<i64>,
    pub favoriteCount: Option<i64>,
    pub zestimateHighPercent: Option<String>,
    pub mlsid: Option<String>,
    pub address: Option<Address>,
    pub city: Option<String>,
    pub providerListingID: Option<String>,
    pub country: Option<String>,
    pub currency: Option<String>,
    pub contingentListingType: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct MortgageRates {
    arm5Rate: Option<f64>,
    fifteenYearFixedRate: Option<f64>,
    thirtyYearFixedRate: Option<f64>,
}

#[derive(Deserialize, Debug)]
pub struct Address {
    pub city: Option<String>,
    pub neighborhood: Option<String>,
    pub state: Option<String>,
    pub streetAddress: Option<String>,
    pub zipcode: Option<String>,
}

pub async fn get_zillow_property_results_by_zpid(
    config: Arc<Config>,
    reqwest_client: Arc<reqwest::Client>,
    zpid: String,
) -> Result<ZillowPropertySearchRoot, OstrichError> {
    let api_url = format!(
        "https://{}/property?zpid={}",
        config.zillow_api.api_host, zpid
    );
    log::info!("Getting property at {}", api_url);

    let listing_data = reqwest_client
        .get(api_url)
        .header("X-RapidAPI-Host", config.zillow_api.api_host.clone())
        .header("X-RapidAPI-Key", config.zillow_api.api_key.clone())
        .send()
        .await?
        .json::<ZillowPropertySearchRoot>()
        .await
        .map_err(|e| {
            OstrichError::new(
                format!(
                    "Zillow Property {} did not result in a valid response {:?}",
                    zpid, e
                ),
                OstrichErrorType::PropertyResultError,
            )
        })?;

    Ok(listing_data)
}

pub async fn get_listing_email_for_search_params(
    config: Arc<Config>,
    db_conn: Arc<DbConn>,
    reqwest_client: Arc<reqwest::Client>,
    emailer_record: &Emailer,
    body: String,
    delay: u64,
) -> Result<String, OstrichError> {
    let api_url =
        get_zillow_listing_url_from_params(config.clone(), &emailer_record.into(), Some(1));
    let listing_results =
        get_zillow_listing_results(config.clone(), reqwest_client.clone(), api_url).await?;

    log::debug!("listing_results = {:?}", listing_results);

    let zpids = listing_results
        .props
        .into_iter()
        .map(|listing| listing.zpid)
        .collect::<Vec<Option<String>>>();

    log::info!("Found {} properties", zpids.len());

    let email_body = stream::iter(zpids)
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .then(|zpid|
            get_zillow_property_results_by_zpid(config.clone(), reqwest_client.clone(), zpid)
        )
        .filter(|x| {
            if x.is_ok() {
                log::debug!("{:?}", x);
                true
            } else {
                log::error!("{:?}", x);
                false
            }
        })
        .map(|x| x.unwrap())
        .map(|property_result| {
            NewListingData::new(property_result, emailer_record)
        })
        .map(|zillow_email_data| {
            zillow_email_data.insert(&db_conn.get_conn());
            let formatted_property_string = zillow_email_data.to_email();
            log::debug!("{:?}", formatted_property_string);
            formatted_property_string
        })
        .fold(body, |acc, listing_data| format!("{}<div style=\"border-top:1px solid black;\">{}</div>", acc, listing_data))
        .await;
    
    Ok(email_body)
}
