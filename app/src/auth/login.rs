use leptos::{component, create_local_resource, create_server_action, create_signal, view, IntoView,SignalWith};
use leptos_meta::{provide_meta_context, Title};
use leptos_router::{ActionForm};
use super::auth_style;
use leptos::event_target_value;
use crate::header::SimpleHeader;
use super::api::{LoginApi, login_exists};

#[component]
pub fn login() -> impl IntoView {
    provide_meta_context();
    let login = create_server_action::<LoginApi>();

    let (login_read, login_write) = create_signal(String::new());
    let (password_read, password_write) = create_signal(String::new());
    let login_valid_resource = create_local_resource(
        login_read,
        move |val| async move {
            login_exists(val).await
        }
    );
    // let user_valid_loading = user_valid_resource.loading();
    let login_valid_class = move || {
        // if user_valid_loading() {
        //     return "warning";
        // }
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
        <Title text="Login" />
        <div class=auth_style::authcontainer>
            <SimpleHeader title="Login".to_string()/>
            <main>
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
            </main>
        </div>
    }
}