use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
use leptos_router::components::A;

stylance::import_style!(style,"./home.module.scss");
#[component]
pub fn HomePage() -> impl IntoView {
    provide_meta_context();

    view! {
        <head>
            <Title text="Home" />
        </head>
        <div class=style::maincontainer>
            <h1>"Z80 "<span>"Emulator"</span></h1>
            <div class=style::buttoncontainer>
                <A href="login">"Login"</A>
                <A href="register">"Register"</A>
                <A href="emulator/z80">"Guest"</A>
            </div>
        </div>
    }
}