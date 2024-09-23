// @generated automatically by Diesel CLI.

diesel::table! {
    saved_roms (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        #[max_length = 50]
        name -> Varchar,
        description -> Nullable<Text>,
        data -> Bytea,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    saved_states (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        #[max_length = 50]
        name -> Varchar,
        description -> Nullable<Text>,
        data -> Bytea,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    sessions (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 150]
        token -> Varchar,
        created_at -> Timestamp,
        expires_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 254]
        email -> Varchar,
        #[max_length = 150]
        password_hash -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(saved_roms -> users (user_id));
diesel::joinable!(saved_states -> users (user_id));
diesel::joinable!(sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(saved_roms, saved_states, sessions, users,);
