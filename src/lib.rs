#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod config;
pub mod db_conn;
pub mod error;
pub mod handlers;
pub mod models;
pub mod routes;
pub mod schema;
pub mod services;
pub mod utils;

use crate::{
    config::Config,
    db_conn::DbConn,
    utils::{decode_bearer, JwtPayload},
};
use aws_sdk_cognitoidentityprovider::Client as CognitoClient;
use serde::Serialize;
use std::error::Error;
use std::{convert::Infallible, sync::Arc};
use warp::{
    filters::body::BodyDeserializeError, filters::BoxedFilter, http::StatusCode, Filter, Rejection,
    Reply,
};

// A simple type alias so as to DRY.
pub type DynResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub fn with_config(config: Arc<Config>) -> BoxedFilter<(Arc<Config>,)> {
    warp::any().map(move || config.clone()).boxed()
}

pub fn with_cognito(cognito: Arc<CognitoClient>) -> BoxedFilter<(Arc<CognitoClient>,)> {
    warp::any().map(move || cognito.clone()).boxed()
}

pub fn with_db_conn(conn: Arc<DbConn>) -> warp::filters::BoxedFilter<(Arc<DbConn>,)> {
    warp::any().map(move || conn.clone()).boxed()
}

async fn with_jwt(jwt: String) -> Result<JwtPayload, warp::Rejection> {
    decode_bearer(&jwt).map_err(|_| warp::reject::custom(BadJwt))
}

pub fn with_reqwest_client(
    client: Arc<reqwest::Client>,
) -> warp::filters::BoxedFilter<(Arc<reqwest::Client>,)> {
    warp::any().map(move || client.clone()).boxed()
}

pub async fn get_user_service_health(config: Arc<Config>) -> Result<impl Reply, Infallible> {
    match services::user::ping_user_service(config).await {
        Ok(_) => log::info!("Healthy user service"),
        Err(e) => log::error!("Error! user service, {:?}", e),
    }
    Ok(warp::reply())
}

// An API error serializable to JSON.
#[derive(Serialize)]
pub struct ErrorMessage {
    code: u16,
    message: String,
}

#[derive(Serialize)]
pub struct SuccessMessage {
    code: u16,
    message: String,
}

#[derive(Debug)]
struct BadJwt;
impl warp::reject::Reject for BadJwt {}

// This function receives a `Rejection` and tries to return a custom
// value, otherwise simply passes the rejection along.
pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = String::from("NOT_FOUND");
    } else if let Some(e) = err.find::<services::cognito::CognitoError>() {
        code = StatusCode::BAD_REQUEST;
        message = e.cause.clone();
    } else if let Some(e) = err.find::<error::OstrichError>() {
        code = StatusCode::BAD_REQUEST;
        message = e.details.clone();
    } else if let Some(_) = err.find::<BadJwt>() {
        code = StatusCode::BAD_REQUEST;
        message = String::from("JWT was not well formed!");
    } else if let Some(e) = err.find::<BodyDeserializeError>() {
        // This error happens if the body could not be deserialized correctly
        // We can use the cause to analyze the error and customize the error message
        message = match e.source() {
            Some(cause) => cause.to_string(),
            None => String::from("BAD_REQUEST"),
        };
        code = StatusCode::BAD_REQUEST;
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        // We can handle a specific error, here METHOD_NOT_ALLOWED,
        // and render it however we want
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = String::from("METHOD_NOT_ALLOWED");
    } else {
        // We should have expected this... Just log and say its a 500
        log::error!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = String::from("UNHANDLED_REJECTION");
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}

pub fn handle_succcess_message(message: String) -> impl warp::Reply {
    warp::reply::json(&SuccessMessage {
        code: StatusCode::OK.as_u16(),
        message,
    })
}
