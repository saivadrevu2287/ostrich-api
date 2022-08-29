use crate::{
    models::user::{self, User},
    utils::JwtPayload,
    DbConn,
};
use std::sync::Arc;

pub async fn with_user(
    jwt: JwtPayload,
    db_conn: Arc<DbConn>,
) -> Result<(JwtPayload, Arc<DbConn>, User), warp::Rejection> {
    let conn = db_conn.get_conn();
    let user = user::get_user_by_authentication_id(&conn, jwt.sub.clone())
        .first()
        .cloned()
        .ok_or(warp::reject::not_found())?;
    Ok((jwt, db_conn, user))
}
