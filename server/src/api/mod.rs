mod user;

use crate::middleware;
use axum::middleware::from_fn;
use axum::Router;
use diesel::r2d2::ConnectionManager;
use diesel::{r2d2, PgConnection};
use emulator::emulator_routes;
use leptos::LeptosOptions;
use serde::{Deserialize, Serialize};
use tower_http::add_extension::AddExtensionLayer;
use user::user_routes;

mod emulator;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Serialize, Deserialize)]
struct StatusMsgResponse {
    success: bool,
    message: String,
}

async fn establish_connection() -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new("postgres://user:pass@localhost/emu_web");
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
pub async fn api_routes() -> Router<LeptosOptions> {
    let pool = establish_connection().await;
    Router::new()
        .nest("/user", user_routes())
        .nest("/emulator", emulator_routes())
        .layer(from_fn(middleware::auth_middleware))
        .layer(AddExtensionLayer::new(pool))
        .fallback(|| async { "Unknown API endpoint!" })
}
