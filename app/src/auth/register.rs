use super::api::{EmailExistsApi, RegisterApi, UserExistsApi};
use super::auth_style;
use crate::auth::login::LoginForm;
use crate::header::SimpleHeader;
use leptos::prelude::*;
use leptos_meta::Title;
use regex::Regex;

fn response_to_class(response:Option<Result<bool,ServerFnError>>)->&'static str{
    match response {
        Some(val) => match val {
            Ok(true) => "invalid",
            Ok(false) => "valid",
            Err(_) => "warning", //error while checking
        },
        None => "", //not checked yet
    }
}

#[island]
pub fn register_form() -> impl IntoView {
    let login = ServerAction::<RegisterApi>::new();
    if let Some(registered) = use_context::<RwSignal<bool>>() {
        Effect::new(move || {
            login.value().with(|val| {
                if let Some(Ok(())) = val {
                    registered.set(true);
                }
            })
        });
    }

    let (username_read, username_write) = signal(String::new());
    let username_exists_action = ServerAction::<UserExistsApi>::new();
    let username_invalid = move || {
        if username_read().len() < 5{
            return Some(Ok(true));
        }
        return username_exists_action.value().get();
    };
    let username_class = move || {
        if username_exists_action.pending().get() {
            return "warning";
        }
        if username_read.with(|username| username.is_empty()) {
            return "";
        }
        response_to_class(username_invalid())
    };
    let (email_read, email_write) = signal(String::new());
    let email_exists_action = ServerAction::<EmailExistsApi>::new();
    let email_invalid = move || {
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        if !email_regex.is_match(&email_read()){
            return Some(Ok(true));
        }
        return email_exists_action.value().get();
    };
    let email_class = move || {
        if email_exists_action.pending().get() {
            return "warning";
        }
        if email_read.with(|email| email.is_empty()) {
            return "";
        }
        response_to_class(email_invalid())
    };

    let (password_read, password_write) = signal(String::new());
    let (verif_password_read, verif_password_write) = signal(String::new());

    let password_valid = move || password_read().len() > 6;
    let verif_password = move || {
        let equal = password_read() == verif_password_read();
        equal && password_valid()
    };

    let allow_submit = move || {
        if let (Some(Ok(false)), Some(Ok(false)), true) = (
            username_invalid(),
            email_exists_action.value().get(),
            verif_password(),
        ) {
            true
        } else {
            false
        }
    };

    view! {
        <ActionForm action=login>
            <div>
                <label for="username">"Username"</label>
                <input
                    type="text"
                    name="username"
                    class=username_class
                    on:input=move |event| {
                        let val = event_target_value(&event);
                        username_write(val.clone());
                        username_exists_action.dispatch(UserExistsApi { username: val });
                    }
                />
            </div>

            <div>
                <label for="email">"Email"</label>
                <input
                    type="email"
                    name="email"
                    class=email_class
                    on:input=move |event| {
                        let val = event_target_value(&event);
                        email_write(val.clone());
                        email_exists_action.dispatch(EmailExistsApi { email: val });
                    }
                />
            </div>

            <div>
                <label for="password">"Password"</label>
                <input
                    type="password"
                    name="password"
                    class=move || {
                        if password_read.with(|pass| pass.is_empty()) {
                            ""
                        } else if password_valid() {
                            "valid"
                        } else {
                            "invalid"
                        }
                    }
                    prop:value=password_read
                    on:input=move |event| {
                        password_write(event_target_value(&event));
                    }
                />
            </div>

            <div>
                <label>"Verify password"</label>
                <input
                    type="password"
                    prop:value=verif_password_read
                    class=move || {
                        if verif_password_read.with(|pass| pass.is_empty()) {
                            ""
                        } else if verif_password() {
                            "valid"
                        } else {
                            "invalid"
                        }
                    }
                    on:input=move |event| {
                        verif_password_write(event_target_value(&event));
                    }
                />
            </div>
            <input
                style:opacity=move || if allow_submit() { "1" } else { "0.5" }
                type="submit"
                value="Register"
            />
        </ActionForm>
    }
}

#[island]
pub fn main_table() -> impl IntoView {
    let registered = RwSignal::new(false);
    provide_context(registered.clone());
    move || -> _ {
        match registered.get() {
            true => view! { <LoginForm /> }.into_any(),
            false => view! { <RegisterForm /> }.into_any(),
        }
    }
}

#[component]
pub fn register() -> impl IntoView {
    view! {
        <Title text="Register" />
        <div class=auth_style::authcontainer>
            <SimpleHeader title="Register".to_string() />
            <main>
                <MainTable />
            </main>
        </div>
    }
}
