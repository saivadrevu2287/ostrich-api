use crate::{schema::emailers, utils::now};

use chrono::naive::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct Emailer {
    id: i32,
    pub search_param: String,
    frequency: String,
    pub email: String,
    authentication_id: String,
    created_at: NaiveDateTime,
    updated_at: Option<NaiveDateTime>,
    deleted_at: Option<NaiveDateTime>,
    active: bool,
}

#[derive(Insertable)]
#[table_name = "emailers"]
pub struct NewEmailer {
    search_param: String,
    authentication_id: String,
    email: String,
    frequency: String,
    created_at: NaiveDateTime,
    updated_at: Option<NaiveDateTime>,
    deleted_at: Option<NaiveDateTime>,
    active: bool,
}

#[derive(Deserialize)]
pub struct PostEmailer {
    pub search_param: String,
    pub frequency: String,
}

impl NewEmailer {
    pub fn new(search_param: String, authentication_id: String, email: String, frequency: String) -> Self {
        NewEmailer {
            search_param,
            authentication_id,
            email,
            frequency,
            created_at: now(),
            updated_at: None,
            deleted_at: None,
            active: true,
        }
    }

    pub fn insert(&self, conn: &PgConnection) -> Emailer {
        create(conn, self)
    }
}

pub fn create(conn: &PgConnection, new_emailer: &NewEmailer) -> Emailer {
    diesel::insert_into(emailers::table)
        .values(new_emailer)
        .get_result(conn)
        .expect("Error saving new emailer")
}

pub fn read(conn: &PgConnection) -> Vec<Emailer> {
    emailers::table
        .load::<Emailer>(conn)
        .expect("Error loading emailer")
}
