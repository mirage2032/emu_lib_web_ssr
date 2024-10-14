// use emu_lib_ui::emulator;

use crate::footer::Footer;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{FlatRoutes, ParentRoute, Route, Router, Routes};
use leptos_router::nested_router::Outlet;
use leptos_router::*;
// use crate::error::AppError;
// use crate::footer::Footer;
use crate::home::HomePage;
use crate::utils::icons::IconsCDN;

mod auth;
mod author;
mod dashboard;
pub mod db;
mod error;
mod footer;
mod header;
mod home;
mod utils;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html> 
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <Stylesheet id="leptos" href="/pkg/start-axum-workspace.css" />
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
        <head>
            <Title formatter=|text| format!("Z80Emu - {}", text) />
        // <Stylesheet id="app" href="/pkg/app.css" />
        </head>
        <main>
            <Router>
                <Routes fallback=|| error::AppError::NotFound.into_view()>
                    // <ParentRoute
                    // path=StaticSegment("emulator")
                    // view=|| {
                    // view! { <Outlet /> }
                    // }
                    // >
                    // <Route path=StaticSegment("z80") view=emulator::Emulator />
                    // </ParentRoute>
                    // <Route path=StaticSegment("emulator/z80") view=emulator::Emulator />
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
