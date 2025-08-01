// All tables and their relationships in the database schema.
// This file is generated by Diesel CLI and should not be edited manually.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "website_status"))]
    pub struct WebsiteStatus;
}

diesel::table! {
    _region_websites (a, b) {
        a -> Text,
        b -> Text,
    }
}

diesel::table! {
    region (id) {
        id -> Text,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    user (id) {
        id -> Text,
        email -> Text,
        password -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    website (id) {
        id -> Text,
        url -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        user_id -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::WebsiteStatus;

    website_tick (id) {
        id -> Text,
        website_id -> Text,
        region_id -> Text,
        response_time -> Int4,
        status -> WebsiteStatus,
        error_message -> Nullable<Text>,
        response_body -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(_region_websites -> region (a));
diesel::joinable!(_region_websites -> website (b));
diesel::joinable!(website -> user (user_id));
diesel::joinable!(website_tick -> region (region_id));
diesel::joinable!(website_tick -> website (website_id));

diesel::allow_tables_to_appear_in_same_query!(
    _region_websites,
    region,
    user,
    website,
    website_tick,
);
