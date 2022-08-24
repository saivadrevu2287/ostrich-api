use crate::{
    config::Config, models::emailer::PostEmailer, services::zillow::ZillowSearchParameters,
    utils::JwtPayload, with_config, with_db_conn, with_jwt, with_reqwest_client, DbConn,
};
use std::sync::Arc;
use warp::{filters::BoxedFilter, Filter};

fn path_prefix() -> BoxedFilter<()> {
    warp::path("emailers").boxed()
}

pub fn get_all_emailers(db_conn: Arc<DbConn>) -> BoxedFilter<(Arc<DbConn>,)> {
    warp::get()
        .and(path_prefix())
        .and(warp::path("all"))
        .and(warp::path::end())
        .and(with_db_conn(db_conn))
        .boxed()
}

pub fn add_emailer(db_conn: Arc<DbConn>) -> BoxedFilter<(JwtPayload, Arc<DbConn>, PostEmailer)> {
    warp::post()
        .and(path_prefix())
        .and(warp::path::end())
        .and(warp::header::<String>("authorization"))
        .and_then(with_jwt)
        .and(with_db_conn(db_conn))
        .and(warp::body::json())
        .boxed()
}

pub fn get_emailer_by_authentication_id(
    db_conn: Arc<DbConn>,
) -> BoxedFilter<(JwtPayload, Arc<DbConn>)> {
    warp::get()
        .and(path_prefix())
        .and(warp::path::end())
        .and(warp::header::<String>("authorization"))
        .and_then(with_jwt)
        .and(with_db_conn(db_conn))
        .boxed()
}

pub fn delete_emailer_by_authentication_id(
    db_conn: Arc<DbConn>,
) -> BoxedFilter<(JwtPayload, Arc<DbConn>, i32)> {
    warp::delete()
        .and(path_prefix())
        .and(warp::header::<String>("authorization"))
        .and_then(with_jwt)
        .and(with_db_conn(db_conn))
        .and(warp::path::param())
        .and(warp::path::end())
        .boxed()
}

pub fn test_emailer_params(
    config: Arc<Config>,
    client: Arc<reqwest::Client>,
) -> BoxedFilter<(Arc<Config>, Arc<reqwest::Client>, ZillowSearchParameters)> {
    warp::get()
        .and(path_prefix())
        .and(warp::path("test-search-param"))
        .and(warp::path::end())
        .and(with_config(config))
        .and(with_reqwest_client(client))
        .and(warp::query::<ZillowSearchParameters>())
        .boxed()
}
