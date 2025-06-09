use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_meta::{provide_meta_context, Title};
use leptos_router::components::{Redirect, A};

stylance::import_style!(style, "./home.module.scss");
#[component]
pub fn HomePage() -> impl IntoView {
    provide_meta_context();
    let loadable_resources = Resource::new(
        || (),
        move |_| async move { crate::auth::api::userdata().await },
    );
    let is_connected = move || {
        loadable_resources.with(|val| match val.as_ref() {
            Some(Ok(_)) => true,
            _ => false,
        })
    };
    view! {
        <Transition fallback=move || {}>
            <Show when=is_connected fallback=|| {}>
                <Redirect path="/emulator/z80" />
            </Show>
        </Transition>
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
