use axum::{Json, Router};
use leptos::LeptosOptions;
use crate::api::DbPool;
use crate::models::Users;

fn view_rom(
    id: i32,
    pool: &DbPool,
) -> Result<Json<Users>, diesel::result::Error> {
    let user = Users::get_by_id(id, pool)?;
    Ok(Json(user))
}
pub fn user_routes() -> Router<LeptosOptions> {
    Router::new()
        .route("/register", axum::routing::post(|| async { "Hello, World!" }))
        .route("/login", axum::routing::post(|| async { "Hello, World!" }))
        .route("/logout", axum::routing::post(|| async { "Hello, World!" }))
}