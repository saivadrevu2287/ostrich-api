use crate::{
    config::Config,
    db_conn::DbConn,
    models::{
        emailer::{self, PostEmailer},
        user::get_user_by_authentication_id,
    },
    services::zillow::{self, ZillowSearchParameters},
    utils::JwtPayload,
};
use std::sync::Arc;
use warp::reject;

pub async fn get_all_emailers(db_conn: Arc<DbConn>) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = db_conn.get_conn();
    log::info!("Getting all emailers");
    Ok(warp::reply::json(&emailer::read(&conn)))
}

pub async fn get_all_emailers_from_authentication_id(
    jwt: JwtPayload,
    db_conn: Arc<DbConn>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = db_conn.get_conn();
    log::info!("Getting all emailers for {}", jwt.sub);

    match get_user_by_authentication_id(&conn, jwt.sub).first() {
        None => Err(reject::not_found()),
        Some(user) => Ok(warp::reply::json(&emailer::read_by_user_id(&conn, user.id))),
    }
}

pub async fn delete_emailer_by_id_and_authentication_id(
    jwt: JwtPayload,
    db_conn: Arc<DbConn>,
    id: i32,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = db_conn.get_conn();
    log::info!("Soft deleting emailer for {} with id {}", jwt.sub, id);

    match get_user_by_authentication_id(&conn, jwt.sub).first() {
        None => Err(reject::not_found()),
        Some(user) => Ok(warp::reply::json(&emailer::delete_by_id_and_user_id(
            &conn, id, user.id,
        ))),
    }
}

pub async fn insert_emailer(
    jwt: JwtPayload,
    db_conn: Arc<DbConn>,
    new_emailer: PostEmailer,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = db_conn.get_conn();
    log::info!("Inserting new emailer");

    match get_user_by_authentication_id(&conn, jwt.sub).first() {
        None => Err(reject::not_found()),
        Some(user) => {
            let new_emailer = emailer::NewEmailer::new(new_emailer, user.id, jwt.email);
            Ok(warp::reply::json(&new_emailer.insert(&conn)))
        }
    }
}

pub async fn test_emailer_search_params(
    config: Arc<Config>,
    reqwest_client: Arc<reqwest::Client>,
    test_emailer_params: ZillowSearchParameters,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("Testing {}", test_emailer_params.search_param);

    let api_url =
        zillow::get_zillow_listing_url_from_params(config.clone(), &test_emailer_params, None);

    match zillow::get_zillow_listing_results(config.clone(), reqwest_client.clone(), api_url).await
    {
        Ok(listing_results) => {
            let addresses = listing_results
                .props
                .into_iter()
                .map(|listing| listing.address.map_or(String::from("Missing"), |x| x))
                .collect::<Vec<String>>();

            Ok(warp::reply::json(&addresses))
        }
        Err(e) => Err(warp::reject::custom(e)),
    }
}
