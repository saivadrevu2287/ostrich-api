use crate::config::Config;
use reqwest::Error;
use std::sync::Arc;

pub async fn ping_user_service(config: Arc<Config>) -> Result<(), Error> {
    let user_service_url = format!("http://{}/health", config.user_service_url.clone());
    log::info!("Pinging {}", user_service_url);
    let response = reqwest::Client::new()
        .get(user_service_url)
        .header("X-RapidAPI-Host", config.zillow_api.api_host.clone())
        .header("X-RapidAPI-Key", config.zillow_api.api_key.clone())
        .send()
        .await?
        .text()
        .await?;

    Ok(())
}
