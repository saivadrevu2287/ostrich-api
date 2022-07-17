use aws_config::meta::region::RegionProviderChain;
use aws_sdk_secretsmanager::{Client, Error, Region};

pub async fn get_secrets_manager_client(region: String) -> Client {
    let region_provider = RegionProviderChain::first_try(Region::new(region))
        .or_default_provider()
        .or_else(Region::new("us-west-2"));
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);
    client
}

async fn show_secret(client: &Client, name: &str) -> Result<(), Error> {
    let resp = client.get_secret_value().secret_id(name).send().await?;

    Ok(())
}
