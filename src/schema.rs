table! {
    emailers (id) {
        id -> Int4,
        search_param -> Varchar,
        authentication_id -> Varchar,
        email -> Varchar,
        frequency -> Varchar,
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

allow_tables_to_appear_in_same_query!(
    emailers,
    users,
);
