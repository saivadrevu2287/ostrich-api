use crate::utils::base64_hmac;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_cognitoidentityprovider::{
    error::{
        ConfirmForgotPasswordError, ConfirmSignUpError, ForgotPasswordError, InitiateAuthError,
        ResendConfirmationCodeError, SignUpError,
    },
    model::{AttributeType, AuthFlowType, AuthenticationResultType},
    output::{
        ConfirmForgotPasswordOutput, ConfirmSignUpOutput, ForgotPasswordOutput, InitiateAuthOutput,
        ResendConfirmationCodeOutput, SignUpOutput,
    },
    types::SdkError,
    Client, Region,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use warp::{filters::BoxedFilter, reject, Filter};

// the token data we send back upon login
#[derive(Serialize)]
pub struct AuthenticationDetails {
    pub access_token: Option<String>,
    pub expires_in: i32,
    pub token_type: Option<String>,
    pub refresh_token: Option<String>,
    pub id_token: Option<String>,
}

// convert the aws type to our type
impl From<AuthenticationResultType> for AuthenticationDetails {
    fn from(a: AuthenticationResultType) -> Self {
        Self {
            access_token: a.access_token,
            expires_in: a.expires_in,
            token_type: a.token_type,
            refresh_token: a.refresh_token,
            id_token: a.id_token,
        }
    }
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

pub fn with_cognito(cognito: Arc<Client>) -> BoxedFilter<(Arc<Client>,)> {
    warp::any().map(move || cognito.clone()).boxed()
}

pub async fn get_cognito_client(region: String) -> Client {
    let region_provider = RegionProviderChain::first_try(Region::new(region))
        .or_default_provider()
        .or_else(Region::new("us-west-2"));
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);
    client
}

pub async fn sign_up(
    client: Arc<Client>,
    client_id: String,
    secret_key: String,
    username: String,
    password: String,
    email: String,
) -> Result<SignUpOutput, SdkError<SignUpError>> {
    let email_attribute = AttributeType::builder()
        .set_name(Some("email".to_string()))
        .set_value(Some(String::from(email)))
        .build();

    let message = format!("{}{}", username, client_id);
    let secret_hash = base64_hmac(secret_key, message).expect("Could not accept secret key");

    let sign_up = client
        .sign_up()
        .client_id(client_id)
        .secret_hash(secret_hash)
        .username(username)
        .password(password)
        .set_user_attributes(Some(vec![email_attribute]))
        .send()
        .await;

    sign_up
}

pub async fn verify(
    client: Arc<Client>,
    client_id: String,
    secret_key: String,
    username: String,
    verification_input: String,
) -> Result<ConfirmSignUpOutput, SdkError<ConfirmSignUpError>> {
    let message = format!("{}{}", username, client_id);
    let secret_hash = base64_hmac(secret_key, message).expect("Could not accept secret key");

    let verification = client
        .confirm_sign_up()
        .client_id(client_id)
        .secret_hash(secret_hash)
        .username(username)
        .confirmation_code(verification_input)
        .send()
        .await;

    verification
}

pub async fn resend_confirmation_code(
    client: Arc<Client>,
    client_id: String,
    secret_key: String,
    username: String,
) -> Result<ResendConfirmationCodeOutput, SdkError<ResendConfirmationCodeError>> {
    let message = format!("{}{}", username, client_id);
    let secret_hash = base64_hmac(secret_key, message).expect("Could not accept secret key");

    let resend_code = client
        .resend_confirmation_code()
        .client_id(client_id)
        .secret_hash(secret_hash)
        .username(username)
        .send()
        .await;

    resend_code
}

pub async fn initiate(
    client: Arc<Client>,
    client_id: String,
    secret_key: String,
    username: String,
    password: String,
) -> Result<InitiateAuthOutput, SdkError<InitiateAuthError>> {
    let message = format!("{}{}", username, client_id);
    let secret_hash = base64_hmac(secret_key, message).expect("Could not accept secret key");

    let user_credentials = HashMap::from([
        ("USERNAME".to_string(), username.clone()),
        ("PASSWORD".to_string(), password),
        ("SECRET_HASH".to_string(), secret_hash),
    ]);

    let auth = client
        .initiate_auth()
        .auth_flow(AuthFlowType::UserPasswordAuth)
        .client_id(client_id)
        .set_auth_parameters(Some(user_credentials.clone()))
        .send()
        .await;

    auth
}

pub async fn forgot_password(
    client: Arc<Client>,
    client_id: String,
    secret_key: String,
    username: String,
) -> Result<ForgotPasswordOutput, SdkError<ForgotPasswordError>> {
    let message = format!("{}{}", username, client_id);
    let secret_hash = base64_hmac(secret_key, message).expect("Could not accept secret key");

    let forgot_password = client
        .forgot_password()
        .client_id(client_id)
        .secret_hash(secret_hash)
        .username(username)
        .send()
        .await;

    forgot_password
}

pub async fn confirm_forgot_password(
    client: Arc<Client>,
    client_id: String,
    secret_key: String,
    username: String,
    password: String,
    code: String,
) -> Result<ConfirmForgotPasswordOutput, SdkError<ConfirmForgotPasswordError>> {
    let message = format!("{}{}", username, client_id);
    let secret_hash = base64_hmac(secret_key, message).expect("Could not accept secret key");

    let confirm_forgot_password = client
        .confirm_forgot_password()
        .client_id(client_id)
        .secret_hash(secret_hash)
        .username(username)
        .password(password)
        .confirmation_code(code)
        .send()
        .await;

    confirm_forgot_password
}
