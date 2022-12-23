use crate::{
    services::{email, stripe},
    utils::string_filter,
    with_config, with_db_conn, Config, DbConn,
};
use ::stripe::{Client, WebhookEvent};
use std::sync::Arc;
use warp::{filters::BoxedFilter, Filter};

fn path_prefix() -> BoxedFilter<()> {
    warp::path("webhook").boxed()
}

pub fn webhook(
    config: Arc<Config>,
    db_conn: Arc<DbConn>,
    client: Arc<Client>,
    email_client: Arc<sendgrid_async::Client>,
) -> BoxedFilter<(
    Arc<Config>,
    WebhookEvent,
    Arc<Client>,
    Arc<DbConn>,
    Arc<sendgrid_async::Client>,
)> {
    warp::post()
        .and(path_prefix())
        .and(warp::path::end())
        .and(warp::header::<String>("Stripe-Signature"))
        .and(string_filter(64000))
        .and(with_config(config))
        .and_then(stripe::with_webhook)
        .untuple_one()
        .and(stripe::with_client(client))
        .and(with_db_conn(db_conn))
        .and(email::with_email(email_client))
        .boxed()
}
