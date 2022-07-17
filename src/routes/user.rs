use crate::{with_db_conn, DbConn};
use std::sync::Arc;
use warp::{filters::BoxedFilter, Filter};

fn path_prefix() -> BoxedFilter<()> {
    warp::path("users").boxed()
}

pub fn get_all_users(db_conn: Arc<DbConn>) -> BoxedFilter<(Arc<DbConn>,)> {
    warp::get()
        .and(path_prefix())
        .and(with_db_conn(db_conn))
        .boxed()
}
