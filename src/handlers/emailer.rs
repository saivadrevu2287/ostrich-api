use crate::{
    config::Config,
    db_conn::DbConn,
    models::emailer::{self, PostEmailer},
    services::zillow,
    // utils::JwtPayload,
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct SearchParamQuery {
    search_param: String,
}

pub async fn get_all_emailers(db_conn: Arc<DbConn>) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = db_conn.get_conn();
    log::info!("Getting all emailers");
    Ok(warp::reply::json(&emailer::read(&conn)))
}

pub async fn insert_emailer(
    /*jwt: JwtPayload,*/
    db_conn: Arc<DbConn>,
    new_emailer: PostEmailer,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = db_conn.get_conn();
    log::info!("Inserting new emailer");
    let new_emailer = emailer::NewEmailer::new(
        new_emailer,
        String::from("tester"), //jwt.sub,
    );
    Ok(warp::reply::json(&new_emailer.insert(&conn)))
}

pub async fn test_emailer_search_params(
    config: Arc<Config>,
    reqwest_client: Arc<reqwest::Client>,
    search_params: SearchParamQuery,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("Testing {}", search_params.search_param);
    let api_url = zillow::get_zillow_listing_url_from_search_param(
        config.clone(),
        search_params.search_param,
    );

    let listing_results =
        zillow::get_zillow_listing_results(config.clone(), reqwest_client.clone(), api_url)
            .await
            .unwrap();

    let addresses = listing_results
        .props
        .into_iter()
        .map(|listing| listing.address.map_or(String::from("Missing"), |x| x))
        .collect::<Vec<String>>();

    Ok(warp::reply::json(&addresses))
}
