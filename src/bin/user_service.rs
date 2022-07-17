use env_logger::Env;
use ostrich_api::{config::Config, db_conn::DbConn, handlers, routes};
use std::{net::SocketAddr, sync::Arc};
use warp::Filter;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    log::info!("🥸 Booting up User Service!");

    let config = Arc::new(Config::new(false));
    let db_conn = Arc::new(DbConn::new(&config.db_path));

    let user = routes::user::get_all_users(db_conn.clone()).and_then(handlers::user::get_all_users);
    let emailer = routes::emailer::get_all_emailers(db_conn.clone())
        .and_then(handlers::emailer::get_all_emailers)
        .or(routes::emailer::add_emailer(db_conn.clone())
            .and_then(handlers::emailer::insert_emailer));

    let end = warp::get()
        .and(warp::path("health"))
        .map(|| warp::reply())
        .or(emailer.or(user))
        .with(warp::log("user"));

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
