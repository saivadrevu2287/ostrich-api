use crate::{
    config::Config,
    error::{OstrichError, OstrichErrorType},
};
use reqwest::Error;
use serde_derive::Deserialize;
use std::sync::Arc;
use thousands::Separable;
use tokio::time::{sleep, Duration};

impl From<Error> for OstrichError {
    fn from(e: Error) -> OstrichError {
        OstrichError::new(
            format!("Failed to fetch data {:?}", e),
            OstrichErrorType::ListingError,
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
    pub daysOnZillow: Option<i64>,
    pub price: Option<i64>,
    pub unit: Option<String>,
    pub listingDateTime: Option<String>,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub contingentListingType: Option<String>,
    pub listingStatus: Option<String>,
    pub zpid: Option<String>,
    pub listingSubType: ListingResultsSubType,
    pub imgSrc: Option<String>,
    pub livingArea: Option<i64>,
    pub bathrooms: Option<i64>,
    pub lotAreaUnit: Option<String>,
    pub country: Option<String>,
    pub currency: Option<String>,
    pub bedrooms: Option<i64>,
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

#[derive(Deserialize, Debug)]
pub struct ZillowPropertySearchRoot {
    // listingProvider: Option<String>,
    buildingPermits: Option<String>,
    propertyTaxRate: Option<f64>,
    #[serde(skip)]
    contact_recipients: Option<String>,
    // solarPotential: Option<String>,
    longitude: Option<f64>,
    countyFIPS: Option<String>,
    cityId: Option<i64>,
    timeOnZillow: Option<String>,
    url: Option<String>,
    zestimate: Option<i64>,
    imgSrc: Option<String>,
    zpid: Option<i64>,
    zipcode: Option<String>,
    livingAreaValue: Option<i64>,
    zestimateLowPercent: Option<String>,
    isListedByOwner: Option<bool>,
    propertyTypeDimension: Option<String>,
    #[serde(skip)]
    resoFacts: Option<String>,
    streetAddress: Option<String>,
    county: Option<String>,
    #[serde(skip)]
    taxHistory: Option<String>,
    stateId: Option<i64>,
    countyId: Option<i64>,
    timeZone: Option<String>,
    homeType: Option<String>,
    livingAreaUnits: Option<String>,
    comingSoonOnMarketDate: Option<String>,
    livingArea: Option<i64>,
    bathrooms: Option<i64>,
    annualHomeownersInsurance: Option<i64>,
    state: Option<String>,
    rentZestimate: Option<i64>,
    building: Option<String>,
    brokerId: Option<String>,
    yearBuilt: Option<i64>,
    brokerageName: Option<String>,
    dateSold: Option<String>,
    price: Option<i64>,
    pageViewCount: Option<i64>,
    description: Option<String>,
    mortgageRates: Option<MortgageRates>,
    homeStatus: Option<String>,
    homeFacts: Option<String>,
    latitude: Option<f64>,
    datePosted: Option<String>,
    bedrooms: Option<i64>,
    #[serde(skip)]
    nearbyHomes: Option<String>,
    monthlyHoaFee: Option<i64>,
    #[serde(skip)]
    priceHistory: Option<String>,
    favoriteCount: Option<i64>,
    #[serde(skip)]
    schools: Option<String>,
    zestimateHighPercent: Option<String>,
    mlsid: Option<String>,
    address: Option<Address>,
    city: Option<String>,
    providerListingID: Option<String>,
    country: Option<String>,
    currency: Option<String>,
    #[serde(skip)]
    listed_by: Option<String>,
    contingentListingType: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct MortgageRates {
    arm5Rate: Option<f64>,
    fifteenYearFixedRate: Option<f64>,
    thirtyYearFixedRate: Option<f64>,
}

#[derive(Deserialize, Debug)]
pub struct Address {
    city: Option<String>,
    neighborhood: Option<String>,
    state: Option<String>,
    streetAddress: Option<String>,
    zipcode: Option<String>,
}

/*
curl --request GET \
    --url 'https://zillow-com1.p.rapidapi.com/propertyExtendedSearch?location=redhook%20brooklyn&home_type=Houses' \
    --header 'X-RapidAPI-Host: zillow-com1.p.rapidapi.com' \
    --header 'X-RapidAPI-Key: ****'
*/
pub async fn get_listing_results_from_search_param(
    config: Arc<Config>,
    search_param: String,
) -> Result<ZillowListingsSearchRoot, Error> {
    let api_url = format!(
        "https://{}/propertyExtendedSearch?location={}&home_type=Houses",
        config.zillow_api.api_host, search_param
    );
    log::info!("Getting listings at {}", api_url);

    let listing_data = reqwest::Client::new()
        .get(api_url)
        .header("X-RapidAPI-Host", config.zillow_api.api_host.clone())
        .header("X-RapidAPI-Key", config.zillow_api.api_key.clone())
        .send()
        .await?
        .json::<ZillowListingsSearchRoot>()
        .await?;

    Ok(listing_data)
}

/*
curl --request GET \
    --url 'https://zillow-com1.p.rapidapi.com/property?zpid=31944863' \
    --header 'X-RapidAPI-Host: zillow-com1.p.rapidapi.com' \
    --header 'X-RapidAPI-Key: ****'
*/
pub async fn get_property_results_by_zpid(
    config: Arc<Config>,
    zpid: String,
) -> Result<ZillowPropertySearchRoot, Error> {
    let api_url = format!(
        "https://{}/property?zpid={}",
        config.zillow_api.api_host, zpid
    );
    log::info!("Getting property at {}", api_url);

    let listing_data = reqwest::Client::new()
        .get(api_url)
        .header("X-RapidAPI-Host", config.zillow_api.api_host.clone())
        .header("X-RapidAPI-Key", config.zillow_api.api_key.clone())
        .send()
        .await?
        .json::<ZillowPropertySearchRoot>()
        .await?;

    Ok(listing_data)
}

pub async fn add_property_details_to_body_by_zpid(
    config: Arc<Config>,
    zpid: String,
    mut body: String,
) -> Result<String, String> {
    log::info!("Getting property results for {}", zpid);
    match get_property_results_by_zpid(config.clone(), zpid).await {
        Ok(property_result) => {
            log::debug!("{:?}", property_result);
            let formatted_property_string = format_property_data_for_email(property_result);
            log::debug!("{:?}", formatted_property_string);

            body = format!(
                "{}<div style=\"border-top:1px solid black;\">{}</div>",
                body, formatted_property_string
            );
            Ok(body)
        }
        Err(_) => Err(body),
    }
}

pub fn format_property_data_for_email(property: ZillowPropertySearchRoot) -> String {
    let address = match property.address {
        Some(address) => {
            format!(
                "{} {}, {} {}",
                address.streetAddress.map_or(String::from("Missing"), |x| x),
                address.city.map_or(String::from("Missing"), |x| x),
                address.state.map_or(String::from("Missing"), |x| x),
                address.zipcode.map_or(String::from("Missing"), |x| x)
            )
        }
        None => String::from("MISSING ADDRESS"),
    };

    let specs = format!(
        "Bedrooms: {} | Bathrooms: {}",
        property.bedrooms.map_or(0, |x| x),
        property.bathrooms.map_or(0, |x| x)
    );

    format!(
        "
        <h2>{}</h2>
        <h4>{}</h4>
        <h4>Price: ${}</h4>
        <h4>Renting estimated: ${}</h4>
        <h4>Days on Market: {}</h4>
        <img src=\"{}\">
        <a href=\"https://www.zillow.com{}\">Check it out!</a>
        ",
        address,
        specs,
        property
            .price
            .map_or(String::from("Missing"), |x| x.separate_with_commas()),
        property
            .rentZestimate
            .map_or(String::from("Missing"), |x| x.separate_with_commas()),
        property.timeOnZillow.map_or(String::from("Missing"), |x| x),
        property.imgSrc.map_or(String::from("Missing"), |x| x),
        property.url.map_or(String::from("Missing"), |x| x)
    )
}

pub async fn get_listing_email_for_search_params(
    config: Arc<Config>,
    search_param: String,
    mut body: String,
    delay: u64,
) -> Result<String, OstrichError> {
    let listing_results =
        get_listing_results_from_search_param(config.clone(), search_param.clone()).await?;

    log::debug!("listing_results = {:?}", listing_results);

    let zpids = listing_results
        .props
        .into_iter()
        .map(|listing| listing.zpid)
        .collect::<Vec<Option<String>>>();

    log::info!("Found {} properties", zpids.len());

    for zpid in zpids
        .into_iter()
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
    {
        sleep(Duration::from_millis(delay)).await;
        body = add_property_details_to_body_by_zpid(config.clone(), zpid.clone(), body)
            .await
            .map_err(|body| {
                log::error!("Could not get details for {}!", zpid);
                body
            })
            .unwrap();
    }

    Ok(body)
}
