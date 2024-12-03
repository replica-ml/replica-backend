// @generated automatically by Diesel CLI.

diesel::table! {
    clients (id) {
        id -> Int4,
        client_id -> Varchar,
        client_secret -> Varchar,
        redirect_uri -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    person_models (id) {
        id -> Int4,
        #[max_length = 1024]
        url -> Nullable<Varchar>,
        male -> Nullable<Bool>,
        height_mm -> Nullable<Int4>,
        weight_g -> Nullable<Int4>,
        bust_mm -> Nullable<Int4>,
        waist_mm -> Nullable<Int4>,
        hip_mm -> Nullable<Int4>,
        #[max_length = 10]
        skin_undertone -> Nullable<Varchar>,
        #[max_length = 10]
        skin_colour -> Nullable<Varchar>,
        #[max_length = 10]
        hair_colour -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    profiles (alias) {
        #[max_length = 50]
        alias -> Varchar,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 2048]
        profile_image_url -> Nullable<Varchar>,
        male -> Nullable<Bool>,
        height_mm -> Nullable<Int4>,
        weight_g -> Nullable<Int4>,
        bust_mm -> Nullable<Int4>,
        waist_mm -> Nullable<Int4>,
        hip_mm -> Nullable<Int4>,
        #[max_length = 10]
        skin_undertone -> Nullable<Varchar>,
        #[max_length = 10]
        skin_colour -> Nullable<Varchar>,
        #[max_length = 10]
        hair_colour -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (username) {
        #[max_length = 50]
        username -> Varchar,
        password_hash -> Text,
        role -> Text,
        created_at -> Timestamp,
    }
}

diesel::joinable!(profiles -> users (username));

diesel::allow_tables_to_appear_in_same_query!(clients, person_models, profiles, users,);
