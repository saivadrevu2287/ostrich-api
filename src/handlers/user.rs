use crate::models::user;

pub async fn get_user(user: user::User) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&user))
}
