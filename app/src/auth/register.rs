use leptos::prelude::*;
use leptos_meta::Title;
use super::auth_style;
use regex::Regex;
use crate::auth::login::LoginForm;
use crate::header::SimpleHeader;
use super::api::{RegisterApi};

#[island]
pub fn register_form() -> impl IntoView {
    let login = ServerAction::<RegisterApi>::new();
    // let registered = expect_context::<RwSignal<bool>>();
    // Effect::new(move|| {
    //     login.value().with(|val| {
    //         if let Some(Ok(())) = val {
    //             registered.set(true);
    //         }
    //     })
    // });

    let (username_read, username_write) = signal(String::new());
    let (email_read, email_write) = signal(String::new());
    let (password_read, password_write) = signal(String::new());
    let (verif_password_read, verif_password_write) = signal(String::new());

    // let user_valid_resource = create_local_resource(
    //     (username_read, email_read),
    //     move |(username,email)| async move {
    //         user_exists(val).await
    //     }
    // );
    let username_valid = move || {
        username_read().len() >= 5
    };
    let email_valid = move || {
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        email_regex.is_match(&email_read())

    };
    let password_valid = move || {
        password_read().len() > 6
    };
    let verif_password = move || {
        let equal = password_read() == verif_password_read();
        equal && password_valid()
    };

    let allow_submit = move || {
        username_valid() && email_valid() && password_valid() && verif_password()
    };

    view! {
        <ActionForm action=login>
            <div>
                <label for="username">"Username"</label>
                <input
                    type="text"
                    name="username"
                    class=move || if username_valid() { "valid" } else { "invalid" }
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
                    class=move || if email_valid() { "valid" } else { "invalid" }
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
pub fn register() -> impl IntoView {
    let registered = RwSignal::new(false);
    // provide_context(registered.clone());
    let form = move || -> _ {
        match registered.get() {
            true => {
                view! { <input /> }.into_any() },
            false => { view! { <textarea/> }.into_any() }
        }
    };
    view! {
        // <Title text="Register" />
        <div class=auth_style::authcontainer>
            <SimpleHeader title="Register".to_string() />
        
            <main>{move || form()}</main>
        </div>
    }
}
