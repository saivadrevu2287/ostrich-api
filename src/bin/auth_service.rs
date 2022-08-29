use env_logger::Env;
use ostrich_api::{config::Config, handle_rejection, handlers, routes, services, with_config};
use std::{net::SocketAddr, sync::Arc};
use warp::Filter;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    log::info!("🔍 Booting up the Authentication Service!");

    let config = Arc::new(Config::new(false));
    let cognito =
        Arc::new(services::cognito::get_cognito_client(config.clone().aws_region.clone()).await);

    let login =
        routes::auth::login(config.clone(), cognito.clone()).and_then(handlers::auth::login);

    let sign_up =
        routes::auth::sign_up(config.clone(), cognito.clone()).and_then(handlers::auth::sign_up);

    let verify =
        routes::auth::verify(config.clone(), cognito.clone()).and_then(handlers::auth::verify);

    let resend_code = routes::auth::resend_code(config.clone(), cognito.clone())
        .and_then(handlers::auth::resend_code);

    let forgot_password = routes::auth::forgot_password(config.clone(), cognito.clone())
        .and_then(handlers::auth::forgot_password);

    let confirm_forgot_password =
        routes::auth::confirm_forgot_password(config.clone(), cognito.clone())
            .and_then(handlers::auth::confirm_forgot_password);

    let auth = login
        .or(sign_up)
        .or(verify)
        .or(resend_code)
        .or(forgot_password)
        .or(confirm_forgot_password)
        .recover(handle_rejection);

    let with_control_origin = warp::reply::with::header("Access-Control-Allow-Origin", "*");
    let with_content_allow =
        warp::reply::with::header("Access-Control-Allow-Headers", "Content-Type");

    let health = warp::get().and(warp::path("health")).map(|| warp::reply());

    let end = health
        .or(auth)
        .with(with_control_origin)
        .with(with_content_allow)
        .with(warp::log("auth"));

    let socket_address = config
        .clone()
        .app_addr
        .parse::<SocketAddr>()
        .expect("Could not parse Addr");

    log::info!("Listening at {}", &config.app_addr);

    if config.clone().tls {
        log::info!("TLS Enabled!");

        warp::serve(end)
            .tls()
            .cert_path(config.clone().cert_path.as_ref().unwrap())
            .key_path(config.clone().key_path.as_ref().unwrap())
            .run(socket_address)
            .await;
    } else {
        warp::serve(end).run(socket_address).await;
    }
}
