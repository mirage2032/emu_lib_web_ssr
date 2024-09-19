// @generated automatically by Diesel CLI.

diesel::table! {
    roms (id) {
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
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(roms -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    roms,
    users,
);
