use crate::{
    config::Config,
    error::{OstrichError, OstrichErrorType},
    handlers::emailer::SearchParamQuery,
    models::emailer::Emailer,
    services::cash_on_cash::calculate_coc,
};
use reqwest::Error;
use serde_derive::Deserialize;
use std::sync::Arc;
use thousands::Separable;
use tokio::time::{sleep, Duration};
use urlencoding::encode;

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

pub fn get_zillow_listing_url_from_emailer_record(
    config: Arc<Config>,
    emailer_record: &Emailer,
) -> String {
    let mut api_url = format!(
        //?location=northampton%20county&home_type=Houses&minPrice=100000&maxPrice=200000&daysOn=1
        "https://{}/propertyExtendedSearch?location={}&home_type=Houses&daysOn=1",
        config.zillow_api.api_host,
        encode(&emailer_record.search_param)
    );

    if emailer_record.max_price.is_some() {
        api_url = format!("{}&maxPrice={}", api_url, emailer_record.max_price.unwrap());
    }

    if emailer_record.min_price.is_some() {
        api_url = format!("{}&minPrice={}", api_url, emailer_record.min_price.unwrap());
    }

    api_url
}

pub fn get_zillow_listing_url_from_test_emailer_record(
    config: Arc<Config>,
    test_emailer_params: &SearchParamQuery,
) -> String {
    let mut api_url = format!(
        "https://{}/propertyExtendedSearch?location={}",
        config.zillow_api.api_host,
        encode(&test_emailer_params.search_param)
    );

    if test_emailer_params.max_price.is_some() {
        api_url = format!(
            "{}&maxPrice={}",
            api_url,
            test_emailer_params.max_price.unwrap()
        );
    }

    if test_emailer_params.min_price.is_some() {
        api_url = format!(
            "{}&minPrice={}",
            api_url,
            test_emailer_params.min_price.unwrap()
        );
    }

    api_url
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
    rentZestimate: Option<f64>,
    building: Option<String>,
    brokerId: Option<String>,
    yearBuilt: Option<i64>,
    brokerageName: Option<String>,
    dateSold: Option<String>,
    price: Option<f64>,
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

pub async fn add_property_details_to_body_by_zpid(
    config: Arc<Config>,
    reqwest_client: Arc<reqwest::Client>,
    emailer: &Emailer,
    zpid: String,
    mut body: String,
) -> Result<String, String> {
    match get_zillow_property_results_by_zpid(config.clone(), reqwest_client.clone(), zpid).await {
        Ok(property_result) => {
            log::debug!("{:?}", property_result);
            let formatted_property_string = format_property_data_for_email(
                ZillowPropertyEmailData::new(property_result, emailer),
            );
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

pub struct ZillowPropertyEmailData {
    address: String,
    specs: String,
    price: String,
    rent_estimate: String,
    time_on_zillow: String,
    img_src: String,
    url: String,
    cash_on_cash: String,
}

impl ZillowPropertyEmailData {
    pub fn new(property: ZillowPropertySearchRoot, emailer: &Emailer) -> Self {
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

        let price = property
            .price
            .map_or(String::from("Missing"), |x| x.separate_with_commas());
        let rent_estimate = property
            .rentZestimate
            .map_or(String::from("Missing"), |x| x.separate_with_commas());
        let time_on_zillow = property.timeOnZillow.map_or(String::from("Missing"), |x| x);
        let img_src = property.imgSrc.map_or(String::from("Missing"), |x| x);
        let url = property.url.map_or(String::from("Missing"), |x| x);

        let cash_on_cash = if property.price.is_some()
            && property.rentZestimate.is_some()
            && property.propertyTaxRate.is_some()
        {
            let coc = calculate_coc(
                emailer,
                property.price.unwrap(),
                property.propertyTaxRate.unwrap(),
                property.rentZestimate.unwrap(),
            );
            format!("{:.2}%", coc)
        } else {
            String::from("<a href=\"https://chrome.google.com/webstore/detail/ostrich/aicgkflmidjkbcenllnnlbnfnmicpmgo\">Use Ostrich Plugin to run this calculation!</a>")
        };

        Self {
            address,
            specs,
            price,
            rent_estimate,
            time_on_zillow,
            img_src,
            url,
            cash_on_cash,
        }
    }
}

pub fn format_property_data_for_email(property: ZillowPropertyEmailData) -> String {
    format!(
        "
        <h2>{}</h2>
        <h4>{}</h4>
        <h4>Price: ${}</h4>
        <h4>Cash On Cash: {}</h4>
        <h4>Renting estimated: ${}</h4>
        <h4>Days on Market: {}</h4>
        <img src=\"{}\">
        <a href=\"https://www.zillow.com{}\">Check it out!</a>
        ",
        property.address,
        property.specs,
        property.price,
        property.cash_on_cash,
        property.rent_estimate,
        property.time_on_zillow,
        property.img_src,
        property.url
    )
}

pub async fn get_listing_email_for_search_params(
    config: Arc<Config>,
    reqwest_client: Arc<reqwest::Client>,
    emailer_record: &Emailer,
    mut body: String,
    delay: u64,
) -> Result<String, OstrichError> {
    let api_url = get_zillow_listing_url_from_emailer_record(config.clone(), emailer_record);
    let listing_results =
        get_zillow_listing_results(config.clone(), reqwest_client.clone(), api_url).await?;

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
        body = match add_property_details_to_body_by_zpid(
            config.clone(),
            reqwest_client.clone(),
            emailer_record,
            zpid.clone(),
            body,
        )
        .await
        {
            Err(body) => {
                log::error!("Could not get details for {}!", zpid);
                body
            }
            Ok(body) => body,
        };
    }

    Ok(body)
}
