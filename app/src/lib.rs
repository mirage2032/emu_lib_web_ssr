use crate::error_template::{AppError, ErrorTemplate};

use emu_lib_ui::{
    emu_lib::{cpu::z80::Z80, emulator::Emulator, memory::Memory},
    emulator::emu_with,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::home::HomePage;

pub mod error_template;
mod home;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let (emu_read, emu_write) = create_signal(Emulator::<Z80>::new_w_mem(Memory::new_full_ram()));
    view! {
        <Stylesheet id="leptos" href="/pkg/start-axum-workspace.css" />

        // sets the document title
        <Title text="Z80EMU" />
        // TODO:Header
        <header></header>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors /> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="emulator/z80" view=move || emu_with(emu_read, emu_write) />
                    <Route path="" view=HomePage />
                </Routes>
            </main>
        </Router>
        // TODO:Footer
        <footer></footer>
    }
}