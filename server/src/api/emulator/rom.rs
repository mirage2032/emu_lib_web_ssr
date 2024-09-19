use axum::{Json, Router};
use leptos::LeptosOptions;
pub fn rom_routes() -> Router<LeptosOptions> {
    Router::new()
        .route("/view/:id", axum::routing::get(|| async { "Hello, World!" }))
        .route("/create", axum::routing::post(|| async { "Hello, World!" }))
        .route("/delete/:id", axum::routing::delete(|| async { "Hello, World!" }))
        .route("/update/:id", axum::routing::patch(|| async { "Hello, World!" }))
}