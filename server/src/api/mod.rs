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

#[derive(Serialize, Deserialize)]
struct StatusMsgResponse {
    success: bool,
    message: String,
}
pub async fn api_routes() -> Router<LeptosOptions> {
    Router::new()
        .nest("/user", user_routes())
        .nest("/emulator", emulator_routes())
        .layer(from_fn(middleware::auth_middleware))
        .fallback(|| async { "Unknown API endpoint!" })
}
