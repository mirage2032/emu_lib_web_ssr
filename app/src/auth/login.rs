use leptos_meta::Title;
use super::auth_style;
use leptos::prelude::*;
use crate::header::SimpleHeader;
use super::api::{LoginApi, login_exists};

#[island]
    pub fn LoginForm() -> impl IntoView {
    let login = ServerAction::<LoginApi>::new();
    let login_value = login.value();
    // let navigation = use_navigation();
    // Effect::new(move || {
    //     login.value().with(|val| {
    //         match val {
    //             Some(val) => {
    //                 match val {
    //                     Ok(()) => {
    //                         log!("login success");
    //                     }
    //                     Err(_) => {
    //                         warn!("login failed");
    //                     }
    //                 }
    //             }
    //             None => {}
    //         }
    //     })
    // });
    let (login_read, login_write) = signal(String::new());
    let (password_read, password_write) = signal(String::new());
    let login_valid_resource:Resource<Result<bool,_>> = Resource::new_with_options(
        login_read,
        move |val| async move {
            let exists = login_exists(val).await;
               exists
        },
        false
    );
        let login_valid_class = move || {
            login_valid_resource.with(|val| {
                match val {
                    Some(val) => match val {
                        Ok(true) => "valid",
                        Ok(false) => "invalid",
                        Err(_) => "warning",
                    },
                    None => "warning",
                }
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