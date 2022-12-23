use crate::{
    models::user::User,
    services::{
        email::with_email,
        user::{create_user_from_jwt, with_user},
    },
    with_config, with_db_conn, with_jwt, Config, DbConn,
};
use std::sync::Arc;
use warp::{filters::BoxedFilter, Filter};

fn path_prefix() -> BoxedFilter<()> {
    warp::path("users").boxed()
}

pub fn get_user_by_authentication_id(
    config: Arc<Config>,
    email: Arc<sendgrid_async::Client>,
    db_conn: Arc<DbConn>,
) -> BoxedFilter<(User,)> {
    warp::get()
        .and(path_prefix())
        .and(warp::path::end())
        .and(warp::header::<String>("authorization"))
        .and_then(with_jwt)
        .and(with_db_conn(db_conn.clone()))
        .and_then(with_user)
        .or(warp::get()
            .and(path_prefix())
            .and(warp::path::end())
            .and(warp::header::<String>("authorization"))
            .and_then(with_jwt)
            .and(with_db_conn(db_conn))
            .and(with_config(config))
            .and(with_email(email))
            .and_then(create_user_from_jwt))
        .unify()
        .boxed()
}
