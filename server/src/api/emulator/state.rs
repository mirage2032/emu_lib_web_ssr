use axum::Router;
use leptos::LeptosOptions;

pub fn state_routes() -> Router<LeptosOptions> {
    Router::new()
        .route("/create", axum::routing::post(|| async { "Hello, World!" }))
        .route(
            "/view/:id",
            axum::routing::get(|| async { "Hello, World!" }),
        )
        .route(
            "/delete/:id",
            axum::routing::delete(|| async { "Hello, World!" }),
        )
        .route(
            "/update/:id",
            axum::routing::patch(|| async { "Hello, World!" }),
        )
}
