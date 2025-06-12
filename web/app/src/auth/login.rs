use super::api::{google_login_callback, GoogleLoginCallbackApi, LoginApi, LoginExistsApi};
use super::auth_style;
use super::AuthBackground;
use crate::header::SimpleHeader;
use leptos::attr::defer;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_meta::{Meta, Script, ScriptProps, Title};
use serde_json::to_string;
use std::time::Duration;
use stylance::classes;

#[island]
pub fn LoginForm(public_url:String) -> impl IntoView {
    let login = ServerAction::<LoginApi>::new();
    Effect::new(move || {
        if let Some(Ok(())) = login.value().get().as_ref() {
            let _ = window().location().set_href("/");
        }
    });
    let (password_read, password_write) = signal(String::new());
    let login_valid_action = ServerAction::<LoginExistsApi>::new();
    let login_valid_class = move || {
        if login_valid_action.pending().get() {
            return "warning";
        }
        login_valid_action.value().with(|val| match val.as_ref() {
            Some(val) => match val {
                Ok(true) => "valid",
                Ok(false) => "invalid",
                Err(_) => "warning", //error while checking
            },
            None => "", //not checked yet
        })
    };
    view! {
        <ActionForm action=login>
            <div>
                <label for="login">"Login"</label>
                <input
                    type="text"
                    name="login"
                    class=login_valid_class
                    on:input=move |event| {
                        let val = event_target_value(&event);
                        login_valid_action
                            .dispatch(LoginExistsApi {
                                login: val.clone(),
                            });
                    }
                />
            </div>

            <div>
                <label for="password">"Password"</label>
                <input
                    type="password"
                    name="password"
                    prop:value=password_read
                    on:input=move |event| {
                        password_write(event_target_value(&event));
                    }
                />
            </div>
            <input type="submit" value="Login" />
            <div
                id="g_id_onload"
                data-client_id="652756675182-jij1vm0aiacih2mnhohc51tu32099n85.apps.googleusercontent.com"
                data-ux_mode="redirect"
                data-login_uri=format!("http://{public_url}/api/google_login_callback")
            ></div>
            <div>
                <div class="g_id_signin" data-type="standard"></div>
            </div>
        </ActionForm>
    }
}
#[component]
pub fn Login() -> impl IntoView {
    let public_url = std::env::var("PUBLIC_URL").expect("PUBLIC_URL");
    view! {
        <Title text="Login" />
        <Script src="https://accounts.google.com/gsi/client" defer="defer" async_="async" />
        <div class=auth_style::authcontainer>
            <SimpleHeader title="Login".to_string() />
            <div class=auth_style::authmaincontainer>
                // <div class=auth_style::authbgline>
                <AuthBackground />
                // </div>
                <main>
                    <LoginForm public_url=public_url />
                </main>
            </div>
        </div>
    }
}
