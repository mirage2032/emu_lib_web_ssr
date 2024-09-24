#[cfg(not(target_arch = "wasm32"))]
use diesel::{r2d2, PgConnection};
#[cfg(not(target_arch = "wasm32"))]
use diesel::r2d2::ConnectionManager;
pub mod models;
pub mod password;


#[cfg(not(target_arch = "wasm32"))]
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[cfg(not(target_arch = "wasm32"))]
pub async fn establish_connection() -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new("postgres://user:pass@localhost/emu_web");
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

use leptos::LeptosOptions;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub pool: DbPool,
}