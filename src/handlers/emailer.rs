use crate::{
    utils::JwtPayload,
    db_conn::DbConn,
    models::emailer::{self, PostEmailer},
};
use std::sync::Arc;

pub async fn get_all_emailers(db_conn: Arc<DbConn>) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = db_conn.get_conn();
    log::info!("Getting all emailers");
    Ok(warp::reply::json(&emailer::read(&conn)))
}

pub async fn insert_emailer(
    jwt: JwtPayload,
    db_conn: Arc<DbConn>,
    new_emailer: PostEmailer,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = db_conn.get_conn();
    log::info!("Inserting new emailer");
    let new_emailer = emailer::NewEmailer::new(
        new_emailer.search_param,
        jwt.sub,
        jwt.email,
        new_emailer.frequency,
    );
    Ok(warp::reply::json(&new_emailer.insert(&conn)))
}
