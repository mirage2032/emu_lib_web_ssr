// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "usertype"))]
    pub struct Usertype;
}

diesel::table! {
    challenges (id) {
        id -> Int4,
        owner_id -> Nullable<Int4>,
        requirements -> Nullable<Bytea>,
        needs_review -> Bool,
    }
}

diesel::table! {
    programs (id) {
        id -> Int4,
        owner_id -> Nullable<Int4>,
        #[max_length = 50]
        name -> Varchar,
        description -> Nullable<Text>,
        data -> Text,
        compiles -> Bool,
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
    solutions (id) {
        id -> Int4,
        solver_id -> Nullable<Int4>,
        challenge_id -> Nullable<Int4>,
        program_id -> Nullable<Int4>,
        pass_requirements -> Bool,
        grade -> Nullable<Int2>,
    }
}

diesel::table! {
    states (id) {
        id -> Int4,
        owner_id -> Nullable<Int4>,
        #[max_length = 50]
        name -> Varchar,
        description -> Nullable<Text>,
        data -> Bytea,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Usertype;

    users (id) {
        id -> Int4,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 254]
        email -> Varchar,
        #[max_length = 100]
        oauth_google -> Nullable<Varchar>,
        #[max_length = 100]
        oauth_github -> Nullable<Varchar>,
        #[max_length = 150]
        password_hash -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        user_type -> Usertype,
    }
}

diesel::joinable!(challenges -> users (owner_id));
diesel::joinable!(programs -> users (owner_id));
diesel::joinable!(sessions -> users (user_id));
diesel::joinable!(solutions -> challenges (challenge_id));
diesel::joinable!(solutions -> programs (program_id));
diesel::joinable!(solutions -> users (solver_id));
diesel::joinable!(states -> users (owner_id));

diesel::allow_tables_to_appear_in_same_query!(
    challenges,
    programs,
    sessions,
    solutions,
    states,
    users,
);
