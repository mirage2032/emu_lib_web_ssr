mod state;
mod rom;

use axum::Router;
use leptos::LeptosOptions;

use state::state_routes;
use rom::rom_routes;
pub fn emulator_routes() -> Router<LeptosOptions> {
    Router::new()
        .nest("/state", state_routes())
        .nest("/rom", rom_routes())
}