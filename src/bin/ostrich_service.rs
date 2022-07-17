use env_logger::Env;
use ostrich_api::{
    config::Config,
    db_conn::DbConn,
    error::map_ostrich_error,
    services::{email, zillow},
    models::emailer
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), ()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    log::info!("🏛 Booting up the Ostrich Service!");

    let config = Arc::new(Config::new(false));
    let email_client = email::get_email_client(config.clone());

    let db_conn = Arc::new(DbConn::new(&config.db_path));
    let conn = db_conn.get_conn();

    // hopefully this will keep us around 2 requests per second
    let delay = 700;

    let emailers = emailer::read(&conn);
    for emailer in emailers {
        let search_param = emailer.search_param;
        let to = emailer.email;

        let body = format!(
            "<h1>-Your Daily Zillow Listings-</h1><p>Search Params: {}</p>",
            search_param
        );
    
        log::info!("Running search on {} for {}", search_param, to);
    
        let body = zillow::get_listing_email_for_search_params(
            config.clone(),
            String::from(search_param),
            body,
            delay,
        )
        .await
        .map_err(map_ostrich_error)?;
    
        email::send_zillow_listings_email(&email_client, config.clone(), &to, &body)
            .await
            .map_err(map_ostrich_error)?;
    }

    Ok(())
}
