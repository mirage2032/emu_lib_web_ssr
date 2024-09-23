use leptos::{component, create_server_action, create_signal, view, IntoView};
use leptos_meta::{provide_meta_context, Title};
use leptos_router::{ActionForm};
use super::auth_style;
use leptos::event_target_value;
use regex::Regex;
use crate::header::SimpleHeader;
use super::api::{RegisterApi};

#[component]
pub fn register() -> impl IntoView {
    provide_meta_context();
    let login = create_server_action::<RegisterApi>();

    let (username_read, username_write) = create_signal(String::new());
    let (email_read, email_write) = create_signal(String::new());
    let (password_read, password_write) = create_signal(String::new());
    let (verif_password_read, verif_password_write) = create_signal(String::new());

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
        <Title text="Register" />
        <div class=auth_style::authcontainer>
            <SimpleHeader title="Register".to_string()/>
            <main>
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
                    type="submit" value="Register" />
                </ActionForm>
            </main>
        </div>
    }
}