use crate::{
    handle_succcess_message,
    services::{
        self,
        cognito::{self as cognito_service},
    },
    Config,
};
use aws_sdk_cognitoidentityprovider::types::SdkError;
use aws_sdk_cognitoidentityprovider::Client as CognitoClient;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use warp::reject;

// the token data we send back upon login
#[derive(Serialize)]
pub struct AuthenticationDetails {
    pub access_token: Option<String>,
    pub expires_in: i32,
    pub token_type: Option<String>,
    pub refresh_token: Option<String>,
    pub id_token: Option<String>,
}

// post body when logging in
#[derive(Deserialize)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String,
}

// post body when confirming your email
#[derive(Deserialize)]
pub struct ConfirmationCredentials {
    pub username: String,
    pub code: String,
}

// post body when running forgot password
#[derive(Deserialize, Serialize)]
pub struct UsernameCredentials {
    pub username: String,
}

// post body when confirming your forgotten password
#[derive(Deserialize, Serialize)]
pub struct ConfirmForgotPasswordCredentials {
    pub username: String,
    pub password: String,
    pub code: String,
}

// post body when refreshing the tokens
#[derive(Deserialize, Serialize)]
pub struct RefreshCredentials {
    pub username: String,
    pub refresh_token: String,
}

// error type that we send back to user
#[derive(Debug)]
pub struct CognitoError {
    pub cause: String,
}

impl reject::Reject for CognitoError {}

// wrapper to send back cognito errors to our user
pub fn reject_with_cognito_error(message: &str) -> warp::Rejection {
    reject::custom(CognitoError::new(String::from(message)))
}

impl CognitoError {
    pub fn new(cause: String) -> Self {
        CognitoError { cause }
    }
}

pub async fn login(
    login_credentials: LoginCredentials,
    config: Arc<Config>,
    cognito: Arc<CognitoClient>,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("Login from {}", login_credentials.username);
    let login_results = cognito_service::initiate(
        cognito,
        format!("{}", config.cognito.client_id),
        format!("{}", config.cognito.secret_key),
        login_credentials.username,
        login_credentials.password,
    )
    .await;

    match login_results {
        Err(error) => Err(handle_cognito_error(error)),
        Ok(login_result) => match login_result.authentication_result {
            Some(authentication_result) => Ok(warp::reply::json(&AuthenticationDetails::from(
                authentication_result,
            ))),
            None => Err(reject_with_cognito_error("No Authentication Results")),
        },
    }
}

pub async fn sign_up(
    login_credentials: LoginCredentials,
    config: Arc<Config>,
    cognito: Arc<CognitoClient>,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("SignUp from {}", login_credentials.username);
    let sign_up_results = cognito_service::sign_up(
        cognito,
        format!("{}", config.cognito.client_id),
        format!("{}", config.cognito.secret_key),
        login_credentials.username.clone(),
        login_credentials.password,
        login_credentials.username.clone(),
    )
    .await;

    match sign_up_results {
        Err(error) => Err(handle_cognito_error(error)),
        // here we could check if the user is confirmed already
        Ok(sign_up_result) => match sign_up_result.user_sub {
            Some(_) => {
                log::info!("Creating new user {}", login_credentials.username);
                Ok(handle_succcess_message(format!(
                    "SIGN_UP_SUCCESSFULL: {}",
                    login_credentials.username
                )))
            }
            None => Err(reject_with_cognito_error("No Sign Up Results")),
        },
    }
}

pub async fn verify(
    confirmation_credentials: ConfirmationCredentials,
    config: Arc<Config>,
    cognito: Arc<CognitoClient>,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("Confirmation from {}", confirmation_credentials.username);
    let verify_results = cognito_service::verify(
        cognito,
        format!("{}", config.cognito.client_id),
        format!("{}", config.cognito.secret_key),
        confirmation_credentials.username.clone(),
        confirmation_credentials.code,
    )
    .await;

    verify_results
        .map_err(handle_cognito_error)
        .map(|_| Ok(handle_succcess_message(format!("USER_VERIFIED"))))
}

pub async fn resend_code(
    username_credentials: UsernameCredentials,
    config: Arc<Config>,
    cognito: Arc<CognitoClient>,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("Resend Confirmation from {}", username_credentials.username);
    let resend_results = cognito_service::resend_confirmation_code(
        cognito,
        format!("{}", config.cognito.client_id),
        format!("{}", config.cognito.secret_key),
        username_credentials.username,
    )
    .await;

    resend_results
        .map_err(handle_cognito_error)
        .map(|_| Ok(handle_succcess_message(format!("CONFIRMATION_RESENT"))))
}

pub async fn forgot_password(
    username_credentials: UsernameCredentials,
    config: Arc<Config>,
    cognito: Arc<CognitoClient>,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("Forgot password from {}", username_credentials.username);
    let resend_results = cognito_service::forgot_password(
        cognito,
        format!("{}", config.cognito.client_id),
        format!("{}", config.cognito.secret_key),
        username_credentials.username,
    )
    .await;

    resend_results
        .map_err(handle_cognito_error)
        .map(|_| Ok(handle_succcess_message(format!("PASSWORD_RESET"))))
}

pub async fn confirm_forgot_password(
    confirm_forgot_password_credentials: ConfirmForgotPasswordCredentials,
    config: Arc<Config>,
    cognito: Arc<CognitoClient>,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!(
        "Confirming forgot password from {}",
        confirm_forgot_password_credentials.username
    );
    let resend_results = cognito_service::confirm_forgot_password(
        cognito,
        format!("{}", config.cognito.client_id),
        format!("{}", config.cognito.secret_key),
        confirm_forgot_password_credentials.username,
        confirm_forgot_password_credentials.password,
        confirm_forgot_password_credentials.code,
    )
    .await;

    resend_results
        .map_err(handle_cognito_error)
        .map(|_| Ok(handle_succcess_message(format!("PASSWORD_RESET"))))
}

pub async fn refresh(
    refresh_credentials: RefreshCredentials,
    config: Arc<Config>,
    cognito: Arc<CognitoClient>,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("Login from {}", refresh_credentials.username);
    let login_results = cognito_service::refresh(
        cognito,
        format!("{}", config.cognito.client_id),
        format!("{}", config.cognito.secret_key),
        refresh_credentials.username,
        refresh_credentials.refresh_token,
    )
    .await;

    match login_results {
        Err(error) => Err(handle_cognito_error(error)),
        Ok(login_result) => match login_result.authentication_result {
            Some(authentication_result) => Ok(warp::reply::json(&AuthenticationDetails::from(
                authentication_result,
            ))),
            None => Err(reject_with_cognito_error("No Authentication Results")),
        },
    }
}

// this will either make a new error out of a cognito message
// or provide a catch all message
fn handle_cognito_error<T: std::fmt::Display>(error: SdkError<T>) -> warp::Rejection {
    match error {
        SdkError::ServiceError { err, raw: _ } => reject_with_cognito_error(&format!("{}", err)),
        _ => reject_with_cognito_error("SERVER_ERROR"),
    }
}
