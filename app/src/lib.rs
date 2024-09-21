use crate::error_template::{AppError, ErrorTemplate};

use emu_lib_ui::{
    emu_lib::{cpu::z80::Z80, emulator::Emulator, memory::Memory},
    emulator::emu_with,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
pub mod error_template;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let (emu_read, emu_write) = create_signal(Emulator::<Z80>::new_w_mem(Memory::new_full_ram()));
    view! {
        <Stylesheet id="leptos" href="/pkg/start-axum-workspace.css"/>

        // sets the document title
        <Title text="Z80EMU"/>
        <header>//TODO:Header
        </header>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="emulator/z80" view=move || emu_with(emu_read,emu_write)/>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
        <footer>//TODO:Footer
        </footer>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
