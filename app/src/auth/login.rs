use std::time::Duration;
use codee::string::FromToStringCodec;
use leptos::{component, create_signal, expect_context, server, spawn_local, view, IntoView, ServerFnError};
use leptos_meta::{provide_meta_context, Title};
use leptos_router::{A};
use super::auth_style;
use leptos::event_target_value;
use leptos::logging::warn;
use leptos_use::{use_cookie, use_cookie_with_options, SameSite, UseCookieOptions};
#[server(Login, "/api/login")]
async fn login(login: String, password: String,duration:Duration) -> Result<String, ServerFnError> {
    use crate::db::AppContext;
    use crate::db::models::user::UserLogin;
    let state = expect_context::<AppContext>();
    let pool = state.pool;
    let user = match login.contains('@') {
        true => UserLogin::new_with_email(login, password),
        false => UserLogin::new_with_username(login, password),
    };
    match user.authenticate(&pool, duration){
        Ok((_, session)) => {
            Ok(session.token)
        }
        Err(e) => {
            let msg = format!("Failed to login user: {}", e);
            Err(ServerFnError::Response(msg))
        }
    }
}

#[component]
pub fn Login() -> impl IntoView {
    provide_meta_context();
    let (login_read, login_write) = create_signal(String::new());
    let (password_read, password_write) = create_signal(String::new());
    view! {
        <Title text="Login" />
        <div class=auth_style::authcontainer>
            <header>
                <A href="/">"Home"</A>
                <h1>"Login"</h1>
                <div></div>
            </header>
            <main>
                <div>
                    <div>
                        <label for="login">"Login"</label>
                        <input
                            id="login"
                            type="text"
                            prop:value=login_read
                            on:input=move |event| {
                                login_write(event_target_value(&event));
                            }
                        />
                    </div>
                    <div>
                        <label for="password">"Password"</label>
                        <input
                            id="password"
                            type="password"
                            prop:value=password_read
                            on:input=move |event| {
                                password_write(event_target_value(&event));
                            }
                        />
                    </div>
                    <button on:click=move |_| {
                        let login_field = login_read();
                        let pass_field = password_read();
                        spawn_local(async {
                            let duration = Duration::from_secs(3600 * 24);
                            let session = login(login_field, pass_field,duration).await;
                            match session {
                                Ok(session) => {
                                    let (_, set_cookie) = use_cookie_with_options::<
                                        String,
                                        FromToStringCodec,
                                    >(
                                        "session_token",
                                        UseCookieOptions::default()
                                            .max_age(duration.as_secs() as i64)
                                            .same_site(SameSite::Lax),
                                    );
                                    set_cookie(Some(session));
                                }
                                Err(e) => {
                                    warn!("{}", e);
                                }
                            }
                        });
                    }>"Login"</button>
                </div>
            </main>
        </div>
    }
}