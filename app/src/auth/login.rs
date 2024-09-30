use super::api::{login_exists, LoginApi};
use super::auth_style;
use crate::header::SimpleHeader;
use leptos::prelude::*;
use leptos_meta::Title;

#[island]
pub fn LoginForm() -> impl IntoView {
    let login = ServerAction::<LoginApi>::new();
    let login_val = login.value();

    Effect::new(move || {
        if let Some(Ok(())) = login.value().get() {
            let _ = window().location().set_href("/");
        }
    });

    let (login_read, login_write) = signal(String::new());
    let (password_read, password_write) = signal(String::new());
    let login_valid_resource: Resource<Result<bool, _>> =
        Resource::new(login_read, move |val| async move {
            login_exists(val).await
        });
    let login_valid_class = move || {
        login_valid_resource.with(|val| match val {
            Some(val) => match val {
                Ok(true) => "valid",
                Ok(false) => "invalid",
                Err(_) => "warning",
            },
            None => "warning",
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
                    prop:value=login_read
                    on:input=move |event| {
                        login_write(event_target_value(&event));
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
        // <Show when=move || {
        // if let Some(Ok(())) = login.value().get() {
        // true
        // } else{
        // false
        // }
        // }>
        // <Redirect path="/"/>
        // </Show>
        </ActionForm>
    }
}

#[component]
pub fn Login() -> impl IntoView {
    view! {
        <Title text="Login" />
        <div class=auth_style::authcontainer>
            <SimpleHeader title="Login".to_string() />
            <main>
                <LoginForm />
            </main>
        </div>
    }
}
