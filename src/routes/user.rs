use crate::{utils::JwtPayload, with_db_conn, with_jwt, DbConn};
use std::sync::Arc;
use warp::{filters::BoxedFilter, Filter};

fn path_prefix() -> BoxedFilter<()> {
    warp::path("users").boxed()
}

pub fn get_user_by_authentication_id(
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
