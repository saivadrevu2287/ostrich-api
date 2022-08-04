use crate::{
    config::Config,
    db_conn::DbConn,
    models::emailer::{self, PostEmailer},
    services::zillow,
    utils::JwtPayload,
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct SearchParamQuery {
    pub search_param: String,
    pub max_price: Option<f64>,
    pub min_price: Option<f64>,
    pub no_bedrooms: Option<i32>,
}

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
    Ok(warp::reply::json(&emailer::read_by_authentication_id(
        &conn, jwt.sub,
    )))
}

pub async fn delete_emailer_by_id_and_authentication_id(
    jwt: JwtPayload,
    db_conn: Arc<DbConn>,
    id: i32,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = db_conn.get_conn();
    log::info!("Soft deleting emailer for {} with id {}", jwt.sub, id);
    Ok(warp::reply::json(
        &emailer::delete_by_id_and_authentication_id(&conn, id, jwt.sub),
    ))
}

pub async fn insert_emailer(
    jwt: JwtPayload,
    db_conn: Arc<DbConn>,
    new_emailer: PostEmailer,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = db_conn.get_conn();
    log::info!("Inserting new emailer");
    let new_emailer = emailer::NewEmailer::new(new_emailer, jwt.sub, jwt.email);
    Ok(warp::reply::json(&new_emailer.insert(&conn)))
}

pub async fn test_emailer_search_params(
    config: Arc<Config>,
    reqwest_client: Arc<reqwest::Client>,
    test_emailer_params: SearchParamQuery,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("Testing {}", test_emailer_params.search_param);
    let api_url = zillow::get_zillow_listing_url_from_test_emailer_record(
        config.clone(),
        &test_emailer_params,
    );

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
