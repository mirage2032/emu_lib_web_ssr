use super::api::{email_exists, user_exists, LoginExistsApi, RegisterApi};
use super::auth_style;
use crate::auth::login::LoginForm;
use crate::header::SimpleHeader;
use leptos::prelude::*;
use leptos_meta::Title;
use regex::Regex;

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
    let username_exists_resource: Resource<Result<bool, _>> = Resource::new(
        username_read,
        move |val| async move { user_exists(val).await },
    );

    let (email_read, email_write) = signal(String::new());
    let email_exists_resource: Resource<Result<bool, _>> = Resource::new(
        email_read,
        move |val| async move { email_exists(val).await },
    );

    let (password_read, password_write) = signal(String::new());
    let (verif_password_read, verif_password_write) = signal(String::new());

    let username_valid = move || username_read().len() >= 5;

    let username_class = move || match username_valid() {
        true => match username_exists_resource.get() {
            Some(Ok(true)) => "invalid",
            Some(Ok(false)) => "valid",
            _ => "warning",
        },
        false => "invalid",
    };

    let email_valid = move || {
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        email_regex.is_match(&email_read())
    };

    let email_class = move || match email_valid() {
        true => match email_exists_resource.get() {
            Some(Ok(true)) => "invalid",
            Some(Ok(false)) => "valid",
            _ => "warning",
        },
        false => "invalid",
    };

    let password_valid = move || password_read().len() > 6;
    let verif_password = move || {
        let equal = password_read() == verif_password_read();
        equal && password_valid()
    };

    let allow_submit = move || {
        if let (Some(Ok(false)), Some(Ok(false))) =
            (username_exists_resource.get(), email_exists_resource.get())
        {
        } else {
            return false;
        }
        username_valid() && email_valid() && password_valid() && verif_password()
    };

    view! {
        <ActionForm action=login>
            <div>
                <label for="username">"Username"</label>
                <input
                    type="text"
                    name="username"
                    class=username_class
                    prop:value=username_read
                    on:input=move |event| {
                        username_write(event_target_value(&event));
                    }
                />
            </div>

            <div>
                <label for="email">"Email"</label>
                <input
                    type="email"
                    name="email"
                    class=email_class
                    prop:value=email_read
                    on:input=move |event| {
                        email_write(event_target_value(&event));
                    }
                />
            </div>

            <div>
                <label for="password">"Password"</label>
                <input
                    type="password"
                    name="password"
                    class=move || if password_valid() { "valid" } else { "invalid" }
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
                    class=move || if verif_password() { "valid" } else { "invalid" }
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
