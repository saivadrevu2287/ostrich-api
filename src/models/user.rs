use crate::{schema::users, utils::now};

use chrono::naive::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct User {
    id: i32,
    email: String,
    billing_id: String,
    authentication_id: String,
    created_at: NaiveDateTime,
    updated_at: Option<NaiveDateTime>,
    deleted_at: Option<NaiveDateTime>,
    active: bool,
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
