use axum::Router;
use leptos::LeptosOptions;

pub fn api_routes() -> Router<LeptosOptions> {
    Router::new()
        .route("/create_user", axum::routing::post(|| async { "Hello, World!" }))
        .route("/login", axum::routing::post(|| async { "Hello, World!" }))
        .route("/logout", axum::routing::post(|| async { "Hello, World!" }))

        .route("/states", axum::routing::get(|| async { "Hello, World!" }))
        .route("/save_state", axum::routing::post(|| async { "Hello, World!" }))
        .route("/load_state", axum::routing::get(|| async { "Hello, World!" }))
        .route("/delete_state", axum::routing::delete(|| async { "Hello, World!" }))

        .route("/roms", axum::routing::get(|| async { "Hello, World!" }))
        .route("/save_rom", axum::routing::post(|| async { "Hello, World!" }))
        .route("/load_rom", axum::routing::get(|| async { "Hello, World!" }))
        .route("/delete_rom", axum::routing::delete(|| async { "Hello, World!" }))

        .route("/groups", axum::routing::get(|| async { "Hello, World!" }))
        .route("/create_group", axum::routing::post(|| async { "Hello, World!" }))
        .route("/edit_group", axum::routing::patch(|| async { "Hello, World!" }))
        .route("/delete_group", axum::routing::delete(|| async { "Hello, World!" }))
}