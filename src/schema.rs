table! {
    emailers (id) {
        id -> Int4,
        search_param -> Varchar,
        authentication_id -> Varchar,
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
    }
}

allow_tables_to_appear_in_same_query!(emailers, users,);
