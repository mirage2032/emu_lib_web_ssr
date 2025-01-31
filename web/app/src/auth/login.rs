use std::time::Duration;
use leptos::logging::log;
use super::api::{LoginApi, LoginExistsApi};
use super::auth_style;
use crate::header::SimpleHeader;
use leptos::prelude::*;
use leptos_meta::Title;
use serde_json::to_string;
use stylance::classes;
use super::AuthBackground;

#[island]
pub fn LoginForm() -> impl IntoView {
    let login = ServerAction::<LoginApi>::new();

    Effect::new(move || {
        if let Some(Ok(())) = login.value().get() {
            let _ = window().location().set_href("/dashboard");
        }
    });

    let (password_read, password_write) = signal(String::new());
    let login_valid_action = ServerAction::<LoginExistsApi>::new();
    let login_valid_class = move || {
        if login_valid_action.pending().get() {
            return "warning";
        }
        login_valid_action.value().with(|val| match val {
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
        </ActionForm>
    }
}
#[component]
pub fn Login() -> impl IntoView {
    view! {
        <Title text="Login" />
        <div class=auth_style::authcontainer>
            <SimpleHeader title="Login".to_string() />
            <div class=auth_style::authmaincontainer>
                // <div class=auth_style::authbgline>
                <AuthBackground />
                // </div>
                <main>
                    <LoginForm />
                </main>
            </div>
        </div>
    }
}
