use emu_lib_ui::{
    emu_lib::{cpu::z80::Z80, emulator::Emulator, memory::Memory},
    emulator::emu_with,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::error::AppError;
use crate::footer::Footer;
use crate::home::HomePage;

mod home;
mod error;
mod auth;
//only if not wasm
pub mod db;
#[cfg(not(target_arch = "wasm32"))]
mod server;
mod header;
mod footer;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let (emu_read, emu_write) = create_signal(Emulator::<Z80>::new_w_mem(Memory::new_full_ram()));
    view! {
        <Stylesheet id="leptos" href="/pkg/start-axum-workspace.css" />

        // sets the document title
        <Title formatter=|text| format!("Z80Emu - {}", text) />

        <main>
            <Router fallback=|| { AppError::NotFound.into_view() }>
                <Routes>
                    <Route path="emulator/z80" view=move || emu_with(emu_read, emu_write) />
                    <Route path="login" view=auth::login::Login />
                    <Route path="register" view=auth::register::Register />
                    <Route path="" view=HomePage />
                </Routes>
            </Router>
        </main>
        <Footer />
    }
}