use crate::footer::Footer;
use crate::home::HomePage;
use crate::utils::icons::IconsCDN;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::*;

mod auth;
mod author;
mod dashboard;
pub mod db;
mod emulator;
mod error;
mod footer;
mod header;
mod home;
pub mod utils;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <Stylesheet id="leptos" href="/pkg/start-axum-workspace.css" />
                <link rel="preconnect" href="https://fonts.googleapis.com" />
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
                <link
                    href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:ital,wght@0,100..800;1,100..800&family=Source+Code+Pro:ital,wght@0,200..900;1,200..900&display=swap"
                    rel="stylesheet"
                />
                <HydrationScripts options=options islands=true />
                <IconsCDN />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Title formatter=|text| format!("Z80Emu - {}", text) />
        // <Stylesheet id="app" href="/pkg/app.css" />
        <main>
            <Router>
                <Routes fallback=error::NotFound>
                    // <ParentRoute
                    // path=StaticSegment("emulator")
                    // view=|| {
                    // view! { <Outlet /> }
                    // }
                    // >
                    // <Route path=StaticSegment("z80") view=emulator::Emulator />
                    // </ParentRoute>
                    <Route path=path!("emulator/z80") view=emulator::Emulator />
                    <Route path=path!("login") view=auth::login::Login />
                    <Route path=path!("register") view=auth::register::Register />
                    <Route path=path!("dashboard") view=dashboard::Dashboard />
                    <Route path=path!("author") view=author::Author />
                    <Route path=path!("") view=HomePage />
                </Routes>
            </Router>
        </main>
        <Footer />
    }
}
