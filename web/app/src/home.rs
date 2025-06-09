use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
use leptos_router::components::A;

stylance::import_style!(style, "./home.module.scss");
#[component]
pub fn HomePage() -> impl IntoView {
    provide_meta_context();
    view! {
        <head>
            <Title text="Home" />
        </head>
        <div class=style::maincontainer>
            <div class=style::title>
                <h1>"Z80"</h1>
                <span>Emulator</span>
            </div>
            <div class=style::buttoncontainer>
                <A href="login">"Login"</A>
                <A href="register">"Register"</A>
                <A href="emulator/z80">"Emulator"</A>
            </div>
        </div>
    }
}
