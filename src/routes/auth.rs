use crate::{
    handlers::auth::{
        ConfirmForgotPasswordCredentials, ConfirmationCredentials, LoginCredentials,
        RefreshCredentials, UsernameCredentials,
    },
    services::cognito::with_cognito,
    with_config, Config,
};
use aws_sdk_cognitoidentityprovider::Client;
use std::sync::Arc;
use warp::{filters::BoxedFilter, Filter};

pub fn login(
    config: Arc<Config>,
    cognito: Arc<Client>,
) -> BoxedFilter<(LoginCredentials, Arc<Config>, Arc<Client>)> {
    warp::post()
        .and(warp::path("login"))
        .and(warp::body::json())
        .and(with_config(config))
        .and(with_cognito(cognito))
        .boxed()
}

pub fn sign_up(
    config: Arc<Config>,
    cognito: Arc<Client>,
) -> BoxedFilter<(LoginCredentials, Arc<Config>, Arc<Client>)> {
    warp::post()
        .and(warp::path("sign-up"))
        .and(warp::body::json())
        .and(with_config(config))
        .and(with_cognito(cognito))
        .boxed()
}

pub fn verify(
    config: Arc<Config>,
    cognito: Arc<Client>,
) -> BoxedFilter<(ConfirmationCredentials, Arc<Config>, Arc<Client>)> {
    warp::post()
        .and(warp::path("verify"))
        .and(warp::body::json())
        .and(with_config(config))
        .and(with_cognito(cognito))
        .boxed()
}

pub fn resend_code(
    config: Arc<Config>,
    cognito: Arc<Client>,
) -> BoxedFilter<(UsernameCredentials, Arc<Config>, Arc<Client>)> {
    warp::post()
        .and(warp::path("resend-code"))
        .and(warp::body::json())
        .and(with_config(config))
        .and(with_cognito(cognito))
        .boxed()
}

pub fn forgot_password(
    config: Arc<Config>,
    cognito: Arc<Client>,
) -> BoxedFilter<(UsernameCredentials, Arc<Config>, Arc<Client>)> {
    warp::post()
        .and(warp::path("forgot-password"))
        .and(warp::body::json())
        .and(with_config(config))
        .and(with_cognito(cognito))
        .boxed()
}

pub fn confirm_forgot_password(
    config: Arc<Config>,
    cognito: Arc<Client>,
) -> BoxedFilter<(ConfirmForgotPasswordCredentials, Arc<Config>, Arc<Client>)> {
    warp::post()
        .and(warp::path("confirm-forgot-password"))
        .and(warp::body::json())
        .and(with_config(config))
        .and(with_cognito(cognito))
        .boxed()
}

pub fn refresh_token(
    config: Arc<Config>,
    cognito: Arc<Client>,
) -> BoxedFilter<(RefreshCredentials, Arc<Config>, Arc<Client>)> {
    warp::post()
        .and(warp::path("refresh"))
        .and(warp::body::json())
        .and(with_config(config))
        .and(with_cognito(cognito))
        .boxed()
}
