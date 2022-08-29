table! {
    emailers (id) {
        id -> Int4,
        search_param -> Varchar,
        email -> Varchar,
        frequency -> Varchar,
        max_price -> Nullable<Float8>,
        min_price -> Nullable<Float8>,
        no_bedrooms -> Nullable<Int4>,
        insurance -> Float8,
        vacancy -> Float8,
        property_management -> Float8,
        capex -> Float8,
        repairs -> Float8,
        utilities -> Float8,
        down_payment -> Float8,
        closing_cost -> Float8,
        loan_interest -> Float8,
        loan_months -> Float8,
        additional_monthly_expenses -> Float8,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        active -> Bool,
        user_id -> Int4,
        no_bathrooms -> Nullable<Int4>,
        notes -> Nullable<Varchar>,
    }
}

table! {
    listing_data (id) {
        id -> Int4,
        user_id -> Int4,
        emailer_id -> Int4,
        street_address -> Nullable<Varchar>,
        city -> Nullable<Varchar>,
        state -> Nullable<Varchar>,
        zipcode -> Nullable<Varchar>,
        bedrooms -> Nullable<Int4>,
        bathrooms -> Nullable<Int4>,
        price -> Nullable<Float8>,
        taxes -> Nullable<Float8>,
        rent_estimate -> Nullable<Float8>,
        time_on_zillow -> Nullable<Varchar>,
        img_src -> Nullable<Varchar>,
        url -> Nullable<Varchar>,
        cash_on_cash -> Nullable<Float8>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        active -> Bool,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        billing_id -> Varchar,
        authentication_id -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        active -> Bool,
        user_tier -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(emailers, listing_data, users,);
