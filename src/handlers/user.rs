use crate::{db_conn::DbConn, models::user};
use std::sync::Arc;

pub async fn get_all_users(db_conn: Arc<DbConn>) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = db_conn.get_conn();
    log::info!("Getting all emailers");
    Ok(warp::reply::json(&user::read(&conn)))
}
