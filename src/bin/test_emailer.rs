use env_logger::Env;
use ostrich_api::{
    config::Config,
    db_conn::DbConn,
    error::{map_ostrich_error, OstrichErrorType},
    models::emailer,
    services::{email, zillow},
    utils,
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), ()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    log::info!("🏛 Booting up the Ostrich Service!");

    let config = Arc::new(Config::new(false));
    let email_client = email::get_email_client(config.clone());
    let reqwest_client = Arc::new(reqwest::Client::new());

    let db_conn = Arc::new(DbConn::new(&config.db_path));

    // hopefully this will keep us around 2 requests per second
    let delay = 700;

    let emailer = emailer::Emailer {
        id: 0,
        user_id: 0,
        search_param: String::from("brooklyn"),
        frequency: String::from("daily"),
        insurance: 60.0,
        vacancy: 5.0,
        property_management: 4.0,
        capex: 5.0,
        repairs: 5.0,
        utilities: 0.0,
        down_payment: 25.0,
        closing_cost: 4.0,
        loan_interest: 4.0,
        loan_months: 240.0,
        additional_monthly_expenses: 0.0,
        no_bedrooms: Some(3),
        max_price: None,
        min_price: None,
        email: String::from("hgmaxwellking@gmail.com"),
        created_at: utils::now(),
        updated_at: None,
        deleted_at: None,
        active: true,
    };

    let search_param = &emailer.search_param;
    let to = &emailer.email;

    let body = format!(
        "<h1>-Your Daily Zillow Listings-</h1><p>Search Params: {}</p>",
        search_param
    );

    log::info!("Running search on {} for {}", search_param, to);

    let res = zillow::get_listing_email_for_search_params(
        config.clone(),
        db_conn.clone(),
        reqwest_client.clone(),
        &emailer,
        body,
        delay,
    )
    .await;
   
    log::info!("{:?}", res);

    Ok(())
}
