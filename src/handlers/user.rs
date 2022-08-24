use crate::{db_conn::DbConn, models::user, utils::JwtPayload};
use std::sync::Arc;

pub async fn get_user_by_authentication_id(
    jwt: JwtPayload,
    db_conn: Arc<DbConn>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = db_conn.get_conn();
    log::info!("Getting user profile for {}", jwt.sub);

    match user::get_user_by_authentication_id(&conn, jwt.sub.clone()).first() {
        None => {
            let new_user = user::NewUser::new(jwt.email, String::from(""), jwt.sub);
            let inserted_user = user::create(&conn, &new_user);
            Ok(warp::reply::json(&inserted_user))
        }
        Some(found_user) => Ok(warp::reply::json(found_user)),
    }
}
