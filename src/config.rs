use dotenv::dotenv;
use log::info;
use std::env;

#[derive(Clone)]
pub struct Config {
    pub app_addr: String,
    pub tls: bool,
    pub cert_path: Option<String>,
    pub key_path: Option<String>,
    pub db_path: String,
    pub is_mocking: bool,
    pub aws_region: String,
    pub cognito: CognitoConfig,
    pub zillow_api: ZillowApiConfig,
    pub email: EmailConfig,
    pub user_service_url: String,
    pub stripe_api: StripeApiConfig,
}

impl Config {
    pub fn new(is_mocking: bool) -> Self {
        info!("🤖 Configuring the application!");
        dotenv().ok();

        // app fields
        let app_host = env::var("HOST").expect("HOST must be set");
        let app_port = env::var("PORT").expect("PORT must be set");
        let app_addr = format!("{}:{}", app_host, app_port);

        // prepare tls if necessary
        let tls = env::var("ENABLE_TLS")
            .expect("ENABLE_TLS must be set")
            .parse()
            .expect("ENABLE_TLS must be true or false");

        let cert_path;
        let key_path;
        if tls {
            cert_path = Some(env::var("CERT_PATH").expect("CERT_PATH must be set"));
            key_path = Some(env::var("KEY_PATH").expect("KEY_PATH must be set"));
        } else {
            cert_path = None;
            key_path = None;
        }

        // url to connect to the database
        let db_path = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let aws_region = env::var("AWS_REGION").expect("AWS_REGION must be set");
        let user_service_url = env::var("USER_SERVICE_URL").expect("USER_SERVICE_URL must be set");

        Config {
            app_addr,
            tls,
            cert_path,
            key_path,
            db_path,
            is_mocking,
            aws_region,
            cognito: CognitoConfig::new(),
            zillow_api: ZillowApiConfig::new(),
            email: EmailConfig::new(),
            stripe_api: StripeApiConfig::new(),
            user_service_url,
        }
    }
}

#[derive(Clone)]
pub struct CognitoConfig {
    pub client_id: String,
    pub secret_key: String,
}

impl CognitoConfig {
    pub fn new() -> Self {
        let client_id = env::var("COGNITO_CLIENT_ID").expect("COGNITO_CLIENT_ID must be set");
        let secret_key = env::var("COGNITO_SECRET_KEY").expect("COGNITO_SECRET_KEY must be set");

        CognitoConfig {
            client_id,
            secret_key,
        }
    }
}

#[derive(Clone)]
pub struct EmailConfig {
    pub api_key: String,
    pub from: String,
}

impl EmailConfig {
    pub fn new() -> Self {
        let api_key = env::var("SENDGRID_API_KEY").expect("SENDGRID_API_KEY must be set");
        let from = env::var("SENDGRID_FROM").expect("SENDGRID_FROM must be set");
        EmailConfig { api_key, from }
    }
}

#[derive(Clone)]
pub struct ZillowApiConfig {
    pub api_host: String,
    pub api_key: String,
}

impl ZillowApiConfig {
    pub fn new() -> Self {
        let api_host = env::var("ZILLOW_API_HOST").expect("ZILLOW_API_HOST must be set");
        let api_key = env::var("ZILLOW_API_KEY").expect("ZILLOW_API_KEY must be set");

        ZillowApiConfig { api_host, api_key }
    }
}

#[derive(Clone)]
pub struct StripeApiConfig {
    pub webhook_signature_secret: String,
    pub client_secret: String,
}

impl StripeApiConfig {
    pub fn new() -> Self {
        let webhook_signature_secret =
            env::var("STRIPE_SIGNATURE_SECRET").expect("STRIPE_SIGNATURE_SECRET must be set");
        let client_secret = env::var("STRIPE_SECRET").expect("STRIPE_SECRET must be set");
        StripeApiConfig {
            webhook_signature_secret,
            client_secret,
        }
    }
}

#[cfg(feature = "mocks")]
pub fn generate_mocking_config() -> Config {
    Config::new(true)
}

pub fn generate_config() -> Config {
    Config::new(false)
}
