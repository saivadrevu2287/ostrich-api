use env_logger::Env;
use ostrich_api::{config::Config, db_conn::DbConn, handle_rejection, handlers, routes, services};
use std::{net::SocketAddr, sync::Arc};
use warp::Filter;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    log::info!("💸 Booting up Stripe Service!");

    let config = Arc::new(Config::new(false));
    let db_conn = Arc::new(DbConn::new(&config.db_path));
    let stripe_client = Arc::new(services::stripe::get_stripe_client(config.clone()));
    let email_client = Arc::new(services::email::get_email_client(config.clone()));

    let health = warp::get().and(warp::path("health")).map(|| warp::reply());

    let webhook = routes::stripe::webhook(
        config.clone(),
        db_conn.clone(),
        stripe_client.clone(),
        email_client.clone(),
    )
    .and_then(handlers::stripe::handle_webhook);

    let end = health
        .or(webhook)
        .recover(handle_rejection)
        .with(warp::log("stripe"));

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
