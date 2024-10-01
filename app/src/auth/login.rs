use super::api::{LoginApi, LoginExistsApi};
use super::auth_style;
use crate::header::SimpleHeader;
use leptos::prelude::*;
use leptos_meta::Title;

#[island]
pub fn LoginForm() -> impl IntoView {
    let login = ServerAction::<LoginApi>::new();

    Effect::new(move || {
        if let Some(Ok(())) = login.value().get() {
            let _ = window().location().set_href("/dashboard");
        }
    });

    let (login_read, login_write) = signal(String::new());
    let (password_read, password_write) = signal(String::new());
    let login_valid_action= ServerAction::<LoginExistsApi>::new();
    let login_valid_class = move || {
        if login_valid_action.pending().get() {
            return "warning"
        }
        login_valid_action.value().with(|val| match val {
            Some(val) => match val {
                Ok(true) => "valid",
                Ok(false) => "invalid",
                Err(_) => "warning",//error while checking
            },
            None => "",//not checked yet
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
                        let val = event_target_value(&event);
                        login_valid_action
                            .dispatch(LoginExistsApi {
                                login: val.clone(),
                            });
                        login_write(val);
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
            <main>
                <LoginForm />
            </main>
        </div>
    }
}
