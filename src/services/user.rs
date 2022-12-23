use crate::{
    models::{
        emailer::{self, Emailer},
        user::{self, User},
    },
    services,
    utils::JwtPayload,
    Config, DbConn,
};
use diesel::prelude::*;
use std::sync::Arc;

pub async fn with_user(jwt: JwtPayload, db_conn: Arc<DbConn>) -> Result<User, warp::Rejection> {
    let conn = db_conn.get_conn();
    let user = user::get_user_by_authentication_id(&conn, jwt.sub.clone())
        .first()
        .cloned()
        .ok_or(warp::reject::not_found())?;
    Ok(user)
}

pub async fn with_token_db_and_user(
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

pub async fn create_user_from_jwt(
    jwt: JwtPayload,
    db_conn: Arc<DbConn>,
    config: Arc<Config>,
    email_client: Arc<sendgrid_async::Client>,
) -> Result<User, warp::Rejection> {
    let conn = db_conn.get_conn();
    let new_user = user::NewUser::new(jwt.email.clone(), String::from("Tier 0"), jwt.sub);
    let inserted_user = user::create(&conn, &new_user);
    services::email::email_admin_on_signup(&email_client.clone(), config.clone(), jwt.email).await;
    Ok(inserted_user)
}

pub fn get_emails_for_user_with_tier(
    conn: &PgConnection,
    user_id: i32,
    billing_id: String,
) -> Vec<Emailer> {
    let emailers = emailer::read_by_user_id(conn, user_id);

    if billing_id == "Tier 1" {
        emailers[0..1].to_vec()
    } else if billing_id == "Tier 2" {
        emailers[0..3].to_vec()
    } else if billing_id == "Tier 3" {
        emailers[0..5].to_vec()
    } else {
        vec![]
    }
}
