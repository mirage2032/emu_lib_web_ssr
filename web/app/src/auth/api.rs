use http::HeaderMap;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::server_fn::codec::PostUrl;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(not(target_arch = "wasm32"))]
mod server_imports {
    pub use crate::db::models::user::{EmailNoPasswordLogin, NewUser, User, UserLogin};
    pub use crate::db::AppState;
    pub use crate::utils::cookie::{self, CookieKey};
    pub use axum::extract::RawQuery;
    pub use axum_extra::extract::Query;
    pub use http::StatusCode;
    pub use leptos_axum::extract;
    pub use leptos_axum::ResponseOptions;
}
#[server(LoginApi, endpoint = "/login")]
pub async fn login(login: String, password: String) -> Result<(), ServerFnError> {
    use server_imports::*;
    let response = expect_context::<ResponseOptions>();
    let state = expect_context::<AppState>();
    // let state: Extension<AppState> = extract().await?;
    let pool = &state.pool;
    let duration = time::Duration::seconds(60 * 60 * 24);
    let user_login = UserLogin::new(login, password);
    match user_login.authenticate(&pool, duration) {
        Ok((_, session)) => {
            cookie::server::set(&CookieKey::Session, &session.token, duration, &response)?;
            // leptos_axum::redirect("/");
            Ok(())
        }
        Err(e) => {
            cookie::server::remove(&CookieKey::Session, &response)?;
            response.set_status(StatusCode::UNAUTHORIZED);
            let msg = format!("Failed to login user: {}", e);
            Err(ServerFnError::Response(msg))
        }
    }
}

//marked UNSAFE, just to make sure developer uses it carefully
#[cfg(not(target_arch = "wasm32"))]
pub async unsafe fn login_email_no_password(email: String) -> Result<(), ServerFnError> {
    use server_imports::*;
    let response = expect_context::<ResponseOptions>();
    let state = expect_context::<AppState>();
    let pool = &state.pool;
    let duration = time::Duration::seconds(60 * 60 * 24);
    let email_no_password_login = EmailNoPasswordLogin::new(email.clone());
    match unsafe { email_no_password_login.authenticate(&pool, duration) } {
        Ok((_, session)) => {
            cookie::server::set(&CookieKey::Session, &session.token, duration, &response)?;
            // leptos_axum::redirect("/");
            Ok(())
        }
        Err(e) => {
            cookie::server::remove(&CookieKey::Session, &response)?;
            response.set_status(StatusCode::UNAUTHORIZED);
            let msg = format!("Failed to login user with email: {}: {}", email, e);
            Err(ServerFnError::Response(msg))
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct GoogleCBQuery {
    foo: Option<String>,
    bar: Option<String>,
}

#[allow(non_snake_case)]
#[server(GoogleLoginCallbackApi, endpoint = "/google_login_callback")]
pub async fn google_login_callback(
    clientId: String,
    client_id: String,
    select_by: String,
    g_csrf_token: String,
    credential: String,
) -> Result<(), ServerFnError> {
    use google_oauth::AsyncClient;
    use server_imports::*;
    let oauth_client = AsyncClient::new(clientId);
    let payload = oauth_client
        .validate_id_token(credential)
        .await
        .expect("Could not validate payload");
    if let Some(email) = payload.email {
        if email_exists(email.clone()).await? == true {
            unsafe { login_email_no_password(email).await }
        } else {
            //create random password
            let random_password: String = (0..16).map(|_| rand::random::<char>()).collect();
            if let Some(name) = payload.name {
                if let Ok(()) = register(name, email.clone(), random_password.clone()).await {
                    login(email, random_password).await
                } else {
                    Err(ServerFnError::Response(format!(
                        "Failed to register user with email: {}",
                        email
                    )))
                }
            } else {
                Err(ServerFnError::Response("Name is required".to_string()))
            }
        }
    } else {
        Err(ServerFnError::Response("Email is required".to_string()))
    }
}

#[server(UserExistsApi, endpoint = "/username_exists")]
pub async fn user_exists(username: String) -> Result<bool, ServerFnError> {
    use server_imports::*;
    let state = expect_context::<AppState>();
    let pool = state.pool;

    match User::get_by_username(&username, &pool) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

#[server(EmailExistsApi, endpoint = "/email_exists")]
pub async fn email_exists(email: String) -> Result<bool, ServerFnError> {
    use server_imports::*;
    let state = expect_context::<AppState>();
    let pool = state.pool;
    match User::get_by_email(&email, &pool) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

#[server(LoginExistsApi, endpoint = "/login_exists")]
pub async fn login_exists(login: String) -> Result<bool, ServerFnError> {
    use server_imports::*;
    let state = expect_context::<AppState>();
    // sleep(Duration::from_millis(3000));
    // let headers: HeaderMap = extract().await?;
    // check user session cookie
    // let res = match cookie::server::get(&CookieKey::Session, &headers) {
    //     Some(val) => {
    //         format!("{}", val)
    //     }
    //     _ => "No cookie".to_string(),
    // };
    // log!("log:{}", res);
    // println!("print:{}",res);
    let pool = &state.pool;
    match User::get_by_login(&login, &pool) {
        Ok(Some(_)) => Ok(true),
        Ok(None) => Ok(false),
        Err(err) => Err(ServerFnError::Response(format!(
            "Failed to check login: {}",
            err
        ))),
    }
}

#[server(RegisterApi, endpoint = "/register")]
pub async fn register(
    username: String,
    email: String,
    password: String,
) -> Result<(), ServerFnError> {
    use server_imports::*;
    let response = expect_context::<ResponseOptions>();
    let state = expect_context::<AppState>();
    let pool = &state.pool;
    match NewUser::new(username, email, password) {
        Ok(user) => match User::add_user(user, &pool) {
            Ok(_) => Ok(()),
            Err(e) => {
                response.set_status(StatusCode::BAD_REQUEST);
                let msg = format!("Failed to register user: {}", e);
                leptos::logging::log!("{}", msg);
                Err(ServerFnError::Response(msg))
            }
        },
        Err(e) => {
            response.set_status(StatusCode::BAD_REQUEST);
            let msg = format!("Failed to register user: {}", e);
            Err(ServerFnError::Response(msg))
        }
    }
}
