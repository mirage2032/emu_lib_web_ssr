#[cfg(not(target_arch = "wasm32"))]
use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};
pub mod models;
pub mod password;

#[cfg(not(target_arch = "wasm32"))]
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[cfg(not(target_arch = "wasm32"))]
pub async fn establish_connection() -> DbPool {
    use std::env;
    let db_url = env::var("DB_HOST").expect("DATABASE_URL must be set");
    let db_user = env::var("DB_USER").expect("DATABASE_USER must be set");
    let db_pass = env::var("DB_PASS").expect("DATABASE_PASS must be set");
    let db_name = env::var("DB_NAME").expect("DATABASE_NAME must be set");
    let manager = ConnectionManager::<PgConnection>::new(format!(
        "postgres://{db_user}:{db_pass}@{db_url}/{db_name}"
    ));
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

use leptos::prelude::LeptosOptions;
use crate::db::models::user::UserData;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub pool: DbPool,
    pub reqwest_client: reqwest::Client,
}
