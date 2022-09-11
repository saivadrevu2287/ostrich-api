use crate::{
    models::emailer::Emailer,
    schema::listing_data,
    services::{cash_on_cash::calculate_coc, zillow::ZillowPropertySearchRoot},
    utils::{format_optional_float, format_optional_string, now},
};
use chrono::naive::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct ListingData {
    pub id: i32,
    pub user_id: i32,
    pub emailer_id: i32,
    pub street_address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zipcode: Option<String>,
    pub bedrooms: Option<i32>,
    pub bathrooms: Option<i32>,
    pub price: Option<f64>,
    pub taxes: Option<f64>,
    pub rent_estimate: Option<f64>,
    pub time_on_zillow: Option<String>,
    pub img_src: Option<String>,
    pub url: Option<String>,
    pub cash_on_cash: Option<f64>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub active: bool,
}

#[derive(Insertable)]
#[table_name = "listing_data"]
pub struct NewListingData {
    pub user_id: i32,
    pub emailer_id: i32,
    pub street_address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zipcode: Option<String>,
    pub bedrooms: Option<i32>,
    pub bathrooms: Option<i32>,
    pub price: Option<f64>,
    pub taxes: Option<f64>,
    pub rent_estimate: Option<f64>,
    pub time_on_zillow: Option<String>,
    pub img_src: Option<String>,
    pub url: Option<String>,
    pub cash_on_cash: Option<f64>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub active: bool,
}

impl NewListingData {
    pub fn new(property: ZillowPropertySearchRoot, emailer: &Emailer) -> Self {
        let taxes = if property.propertyTaxRate.is_some()
            && property.price.is_some()
            && property.propertyTaxRate.unwrap() != 0.0
        {
            Some((property.propertyTaxRate.unwrap() * property.price.unwrap()) / 1200.0)
        } else {
            None
        };

        let mut new_email_data = Self {
            emailer_id: emailer.id,
            user_id: emailer.user_id,
            street_address: None,
            city: None,
            state: None,
            zipcode: None,
            bedrooms: property.bedrooms.map(|bedrooms| bedrooms as i32),
            bathrooms: property.bathrooms.map(|bathrooms| bathrooms as i32),
            price: property.price,
            taxes: taxes,
            rent_estimate: property.rentZestimate,
            time_on_zillow: property.timeOnZillow,
            img_src: property.imgSrc,
            url: property.url,
            cash_on_cash: None,
            created_at: now(),
            updated_at: None,
            deleted_at: None,
            active: true,
        };

        if let Some(address) = property.address {
            new_email_data.street_address = address.streetAddress;
            new_email_data.city = address.city;
            new_email_data.state = address.state;
            new_email_data.zipcode = address.zipcode;
        };

        if property.price.is_some() && property.rentZestimate.is_some() && taxes.is_some() {
            let coc = calculate_coc(
                &emailer.into(),
                property.price.unwrap(),
                taxes.unwrap(),
                property.rentZestimate.unwrap(),
            );
            new_email_data.cash_on_cash = Some(coc);
        }

        new_email_data
    }

    pub fn insert(&self, conn: &PgConnection) -> ListingData {
        create(conn, self)
    }

    pub fn to_email(&self) -> String {
        let address = format!(
            "{} {}, {} {}",
            format_optional_string(self.street_address.clone()),
            format_optional_string(self.city.clone()),
            format_optional_string(self.state.clone()),
            format_optional_string(self.zipcode.clone())
        );

        let specs = format!(
            "Bedrooms: {} | Bathrooms: {}",
            self.bedrooms.map_or(0, |x| x),
            self.bathrooms.map_or(0, |x| x)
        );

        let price = format_optional_float(self.price);
        let rent_estimate = format_optional_float(self.rent_estimate);
        let time_on_zillow = format_optional_string(self.time_on_zillow.clone());

        let img_src = format_optional_string(self.img_src.clone());
        let url = format_optional_string(self.url.clone());

        let taxes = format_optional_float(self.taxes);

        let cash_on_cash = if self.cash_on_cash.is_some() {
            format!("{:.2}%", self.cash_on_cash.unwrap())
        } else {
            String::from("<a href=\"https://chrome.google.com/webstore/detail/ostrich/aicgkflmidjkbcenllnnlbnfnmicpmgo\">Use Ostrich Plugin to run this calculation!</a>")
        };

        format!(
            "
            <h2>{}</h2>
            <h4>{}</h4>
            <h4>Price: {}</h4>
            <h4>Taxes: {}</h4>
            <h4>Estimated Rent: {}</h4>
            <h4>Days on Market: {}</h4>
            <h4>Cash On Cash: {}</h4>
            <img src=\"{}\">
            <a href=\"https://www.zillow.com{}\">Check it out!</a>
            ",
            address, specs, price, taxes, rent_estimate, time_on_zillow, cash_on_cash, img_src, url
        )
    }
}

pub fn create(conn: &PgConnection, new_listing_data: &NewListingData) -> ListingData {
    diesel::insert_into(listing_data::table)
        .values(new_listing_data)
        .get_result(conn)
        .expect("Error saving new listing_data")
}

pub fn read(conn: &PgConnection) -> Vec<ListingData> {
    listing_data::table
        .load::<ListingData>(conn)
        .expect("Error loading listing_data")
}
