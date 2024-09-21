mod rom;
mod state;

use axum::Router;
use leptos::LeptosOptions;

use rom::rom_routes;
use state::state_routes;
pub fn emulator_routes() -> Router<LeptosOptions> {
    Router::new()
        .nest("/state", state_routes())
        .nest("/rom", rom_routes())
}
