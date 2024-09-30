use leptos::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
mod server_imports {
    pub use crate::db::models::user::UserData;
    pub use crate::db::models::user::{NewUser, User, UserLogin};
    pub use crate::db::AppState;
    pub use crate::server::{AppCookie, IntoHeaderValue};
    pub use axum::Extension;
    pub use axum_extra::extract::cookie::Cookie;
    pub use http::header::SET_COOKIE;
    pub use http::StatusCode;
    pub use leptos_axum::extract;
    pub use leptos_axum::ResponseOptions;
    pub use std::time::Duration;
}
#[server(LoginApi, endpoint = "/login")]
pub async fn login(login: String, password: String) -> Result<(), ServerFnError> {
    use server_imports::*;
    let response = expect_context::<ResponseOptions>();
    let state = expect_context::<AppState>();
    let pool = state.pool;
    let duration: Duration = Duration::from_secs(3600 * 24);
    let user_login = UserLogin::new(login, password);
    match user_login.authenticate(&pool, duration) {
        Ok((_, session)) => {
            let cookie = Cookie::new_app_cookie(
                "session_token",
                &session.token,
                Duration::from_secs(3600 * 24),
            );
            response.append_header(SET_COOKIE, cookie.into_header_value()?);
            // leptos_axum::redirect("/");
            Ok(())
        }
        Err(e) => {
            let cookie = Cookie::expired_cookie("session_token");
            response.append_header(SET_COOKIE, cookie.into_header_value()?);
            response.set_status(StatusCode::UNAUTHORIZED);
            let msg = format!("Failed to login user: {}", e);
            Err(ServerFnError::Response(msg))
        }
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
    // let user : Result<Extension<UserData>,_> = extract().await;
    let pool = &state.pool;
    match User::get_by_login(&login, &pool) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
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
