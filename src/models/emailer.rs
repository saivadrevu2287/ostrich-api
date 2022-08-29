use crate::{
    schema::emailers,
    services::{cash_on_cash::CashOnCashCalculationParameters, zillow::ZillowSearchParameters},
    utils::now,
};

use chrono::naive::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Identifiable)]
pub struct Emailer {
    pub id: i32,
    pub search_param: String,
    pub email: String,
    pub frequency: String,
    pub max_price: Option<f64>,
    pub min_price: Option<f64>,
    pub no_bedrooms: Option<i32>,
    pub insurance: f64,
    pub vacancy: f64,
    pub property_management: f64,
    pub capex: f64,
    pub repairs: f64,
    pub utilities: f64,
    pub down_payment: f64,
    pub closing_cost: f64,
    pub loan_interest: f64,
    pub loan_months: f64,
    pub additional_monthly_expenses: f64,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub active: bool,
    pub user_id: i32,
    pub no_bathrooms: Option<i32>,
    pub notes: Option<String>,
}

impl Into<ZillowSearchParameters> for &Emailer {
    fn into(self) -> ZillowSearchParameters {
        ZillowSearchParameters {
            search_param: self.search_param.clone(),
            max_price: self.max_price.clone(),
            min_price: self.min_price.clone(),
            no_bedrooms: self.no_bedrooms.clone(),
            no_bathrooms: self.no_bathrooms.clone(),
        }
    }
}

impl Into<CashOnCashCalculationParameters> for &Emailer {
    fn into(self) -> CashOnCashCalculationParameters {
        CashOnCashCalculationParameters {
            insurance: self.insurance,
            vacancy: self.vacancy,
            property_management: self.property_management,
            capex: self.capex,
            repairs: self.repairs,
            utilities: self.utilities,
            down_payment: self.down_payment,
            closing_cost: self.closing_cost,
            loan_interest: self.loan_interest,
            loan_months: self.loan_months,
            additional_monthly_expenses: self.additional_monthly_expenses,
        }
    }
}

#[derive(Insertable)]
#[table_name = "emailers"]
pub struct NewEmailer {
    search_param: String,
    email: String,
    frequency: String,
    max_price: Option<f64>,
    min_price: Option<f64>,
    no_bedrooms: Option<i32>,
    insurance: f64,
    vacancy: f64,
    property_management: f64,
    capex: f64,
    repairs: f64,
    utilities: f64,
    down_payment: f64,
    closing_cost: f64,
    loan_interest: f64,
    loan_months: f64,
    additional_monthly_expenses: f64,
    created_at: NaiveDateTime,
    updated_at: Option<NaiveDateTime>,
    deleted_at: Option<NaiveDateTime>,
    active: bool,
    user_id: i32,
    no_bathrooms: Option<i32>,
    notes: Option<String>,
}

#[derive(AsChangeset, Deserialize)]
#[table_name = "emailers"]
pub struct PutEmailer {
    id: i32,
    search_param: String,
    frequency: String,
    max_price: Option<f64>,
    min_price: Option<f64>,
    no_bedrooms: Option<i32>,
    insurance: f64,
    vacancy: f64,
    property_management: f64,
    capex: f64,
    repairs: f64,
    utilities: f64,
    down_payment: f64,
    closing_cost: f64,
    loan_interest: f64,
    loan_months: f64,
    additional_monthly_expenses: f64,
    updated_at: Option<NaiveDateTime>,
    no_bathrooms: Option<i32>,
    notes: Option<String>,
}

// this is a body that is accept when we are inserting an emailer over POST
#[derive(Deserialize)]
pub struct PostEmailer {
    search_param: String,
    frequency: String,
    max_price: Option<f64>,
    min_price: Option<f64>,
    no_bedrooms: Option<i32>,
    no_bathrooms: Option<i32>,
    insurance: f64,
    vacancy: f64,
    property_management: f64,
    capex: f64,
    repairs: f64,
    utilities: f64,
    down_payment: f64,
    closing_cost: f64,
    loan_interest: f64,
    loan_months: f64,
    additional_monthly_expenses: f64,
    notes: Option<String>,
}

impl NewEmailer {
    pub fn new(post_emailer: PostEmailer, user_id: i32, email: String) -> Self {
        NewEmailer {
            search_param: post_emailer.search_param,
            user_id,
            email,
            frequency: post_emailer.frequency,
            max_price: post_emailer.max_price,
            min_price: post_emailer.min_price,
            no_bedrooms: post_emailer.no_bedrooms,
            no_bathrooms: post_emailer.no_bathrooms,
            insurance: post_emailer.insurance,
            vacancy: post_emailer.vacancy,
            property_management: post_emailer.property_management,
            capex: post_emailer.capex,
            repairs: post_emailer.repairs,
            utilities: post_emailer.utilities,
            down_payment: post_emailer.down_payment,
            closing_cost: post_emailer.closing_cost,
            loan_interest: post_emailer.loan_interest,
            loan_months: post_emailer.loan_months,
            additional_monthly_expenses: post_emailer.additional_monthly_expenses,
            notes: post_emailer.notes,
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
        .filter(emailers::active.eq(true))
        .load::<Emailer>(conn)
        .expect("Error loading emailer")
}

pub fn update_emailer(conn: &PgConnection, mut updated_emailer: PutEmailer, user_id: i32) -> usize {
    updated_emailer.updated_at = Some(now());
    diesel::update(emailers::table)
        .filter(emailers::id.eq(updated_emailer.id))
        .filter(emailers::user_id.eq(user_id))
        .filter(emailers::active.eq(true))
        .set(&updated_emailer)
        .execute(conn)
        .expect("Error updating emailer")
}

pub fn read_by_user_id(conn: &PgConnection, user_id: i32) -> Vec<Emailer> {
    emailers::table
        .filter(emailers::user_id.eq(user_id))
        .filter(emailers::active.eq(true))
        .load::<Emailer>(conn)
        .expect("Error loading emailer")
}

pub fn delete_by_id_and_user_id(conn: &PgConnection, id: i32, user_id: i32) -> Vec<Emailer> {
    diesel::update(emailers::table)
        .set((emailers::active.eq(false), emailers::deleted_at.eq(now())))
        .filter(emailers::user_id.eq(user_id))
        .filter(emailers::id.eq(id))
        .load::<Emailer>(conn)
        .expect("Error soft deleting the email record")
}
