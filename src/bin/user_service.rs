use env_logger::Env;
use ostrich_api::{config::Config, db_conn::DbConn, handle_rejection, handlers, routes};
use std::{net::SocketAddr, sync::Arc};
use warp::Filter;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    log::info!("🥸 Booting up User Service!");

    let config = Arc::new(Config::new(false));
    let db_conn = Arc::new(DbConn::new(&config.db_path));
    let reqwest_client = Arc::new(reqwest::Client::new());

    let with_control_origin = warp::reply::with::header("Access-Control-Allow-Origin", "*");
    let with_content_allow =
        warp::reply::with::header("Access-Control-Allow-Headers", "Content-Type");

    let get_all_emailers = routes::emailer::get_all_emailers(db_conn.clone())
        .and_then(handlers::emailer::get_all_emailers);

    let post_emailer =
        routes::emailer::add_emailer(db_conn.clone()).and_then(handlers::emailer::insert_emailer);

    let test_emailer_params =
        routes::emailer::test_emailer_params(config.clone(), reqwest_client.clone())
            .and_then(handlers::emailer::test_emailer_search_params);

    let get_emailer_by_authentication_id =
        routes::emailer::get_emailer_by_authentication_id(db_conn.clone())
            .and_then(handlers::emailer::get_all_emailers_from_authentication_id);

    let delete_emailer_by_authentication_id =
        routes::emailer::delete_emailer_by_authentication_id(db_conn.clone())
            .and_then(handlers::emailer::delete_emailer_by_id_and_authentication_id);

    let user = routes::user::get_all_users(db_conn.clone()).and_then(handlers::user::get_all_users);

    let emailer = get_all_emailers
        .or(post_emailer)
        .or(test_emailer_params)
        .or(get_emailer_by_authentication_id)
        .or(delete_emailer_by_authentication_id);

    let health = warp::get().and(warp::path("health")).map(|| warp::reply());

    let with_content_headers = warp::reply::with::header(
        "Access-Control-Allow-Headers",
        "Content-Type,authorization,X-Amz-Date,X-Api-Key,X-Amz-Security-Token",
    );
    let with_content_methods = warp::reply::with::header(
        "Access-Control-Allow-Methods",
        "DELETE,GET,HEAD,OPTIONS,PATCH,POST,PUT",
    );
    let cors = warp::options()
        .map(|| warp::reply())
        .with(with_content_headers)
        .with(with_content_methods)
        .with(with_control_origin.clone());

    let end = cors
        .or(health
            .or(emailer)
            .or(user)
            .recover(handle_rejection)
            .with(with_control_origin)
            .with(with_content_allow))
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
