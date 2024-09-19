mod user;

use axum::Router;
use diesel::{r2d2, PgConnection};
use diesel::r2d2::ConnectionManager;
use leptos::LeptosOptions;
use tower_http::add_extension::AddExtensionLayer;
use user::user_routes;
use emulator::emulator_routes;
mod emulator;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

async fn establish_connection() -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new("postgres://user:pass@localhost/emu_web");
    r2d2::Pool::builder().build(manager).expect("Failed to create pool.")
}
pub async fn api_routes() -> Router<LeptosOptions> {
    let pool = establish_connection().await;
    Router::new()
        .nest("/user", user_routes())
        .nest("/emulator", emulator_routes())
        .layer(AddExtensionLayer::new(pool))
        .fallback(|| async { "Unknown API endpoint!" })
}