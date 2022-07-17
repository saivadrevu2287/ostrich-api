use crate::{models::emailer::PostEmailer, with_db_conn, with_jwt, DbConn, utils::JwtPayload};
use std::sync::Arc;
use warp::{filters::BoxedFilter, Filter};

fn path_prefix() -> BoxedFilter<()> {
    warp::path("emailers").boxed()
}

pub fn get_all_emailers(db_conn: Arc<DbConn>) -> BoxedFilter<(Arc<DbConn>,)> {
    warp::get()
        .and(path_prefix())
        .and(with_db_conn(db_conn))
        .boxed()
}

pub fn add_emailer(db_conn: Arc<DbConn>) -> BoxedFilter<(JwtPayload, Arc<DbConn>, PostEmailer)> {
    warp::post()
        .and(path_prefix())
        .and(warp::header::<String>("authorization"))
        .and_then(with_jwt)
        .and(with_db_conn(db_conn))
        .and(warp::body::json())
        .boxed()
}
