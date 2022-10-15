use crate::{schema::users, utils::now};

use chrono::naive::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Serialize, Clone)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub billing_id: String,
    pub authentication_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub active: bool,
    pub user_tier: i32,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    email: String,
    billing_id: String,
    authentication_id: String,
    created_at: NaiveDateTime,
    updated_at: Option<NaiveDateTime>,
    deleted_at: Option<NaiveDateTime>,
    active: bool,
    user_tier: i32,
}

impl NewUser {
    pub fn new(email: String, billing_id: String, authentication_id: String) -> Self {
        NewUser {
            email,
            billing_id,
            authentication_id,
            created_at: now(),
            updated_at: None,
            deleted_at: None,
            active: true,
            user_tier: 0,
        }
    }

    pub fn insert(&self, conn: &PgConnection) -> User {
        create(conn, self)
    }
}

pub fn create(conn: &PgConnection, new_user: &NewUser) -> User {
    diesel::insert_into(users::table)
        .values(new_user)
        .get_result(conn)
        .expect("Error saving new user")
}

pub fn read(conn: &PgConnection) -> Vec<User> {
    users::table.load::<User>(conn).expect("Error loading user")
}

pub fn get_user_by_authentication_id(conn: &PgConnection, authentication_id: String) -> Vec<User> {
    users::table
        .filter(users::authentication_id.eq(authentication_id))
        .filter(users::active.eq(true))
        .load::<User>(conn)
        .expect("Error loading user")
}

pub fn update_user(conn: &PgConnection, email: String, billing_id: String) -> usize {
    diesel::update(users::table)
        .filter(users::email.eq(email))
        .set(users::billing_id.eq(billing_id))
        .execute(conn)
        .expect("Error updating user")
}
