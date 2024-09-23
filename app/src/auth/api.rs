use leptos::{server, ServerFnError};

#[cfg(not(target_arch = "wasm32"))]
mod server_imports {
    pub use crate::server::{AppCookie, IntoHeaderValue};
    pub use leptos_axum::ResponseOptions;
    pub use axum_extra::extract::cookie::Cookie;
    pub use std::time::Duration;
    pub use http::header::SET_COOKIE;
    pub use crate::db::AppState;
    pub use http::StatusCode;
    pub use leptos::{expect_context};
    pub use crate::db::models::user::{UserLogin,User,NewUser};
    pub use axum::Extension;
    pub use crate::db::models::user::UserData;
    pub use leptos_axum::extract;
}
#[server(LoginApi, "/api/login")]
pub async fn login(login: String, password: String) -> Result<(),ServerFnError> {
    use server_imports::*;
    let response = expect_context::<ResponseOptions>();
    let state = expect_context::<AppState>();
    let pool = state.pool;
    let duration: Duration = Duration::from_secs(3600 * 24);
    let user = match login.contains('@') {
        true => UserLogin::new_with_email(login, password),
        false => UserLogin::new_with_username(login, password),
    };
    match user.authenticate(&pool, duration){
        Ok((_, session)) => {
            let cookie = Cookie::new_app_cookie("session_token",
                                                &session.token,
                                                Duration::from_secs(3600 * 24));
            response.append_header(SET_COOKIE,
                                   cookie.into_header_value()?
            );
            Ok(())
        }
        Err(e) => {
            let cookie = Cookie::expired_cookie("session_token");
            response.append_header(SET_COOKIE,
                                   cookie.into_header_value()?
            );
            response.set_status(StatusCode::UNAUTHORIZED);
            let msg = format!("Failed to login user: {}", e);
            Err(ServerFnError::Response(msg))
        }
    }
}

#[server(UserExistsApi, "/api/username_exists")]
pub async fn user_exists(username: String) -> Result<bool,ServerFnError> {
    use server_imports::*;
    let state = expect_context::<AppState>();
    let pool = state.pool;
    match User::get_by_username(&username, &pool) {
        Ok(_) => Ok(true),
        Err(_) => {
            Ok(false)
        }
    }
}

#[server(EmailExistsApi, "/api/email_exists")]
pub async fn email_exists(email: String) -> Result<bool,ServerFnError> {
    use server_imports::*;
    let state = expect_context::<AppState>();
    let pool = state.pool;
    match User::get_by_email(&email, &pool) {
        Ok(_) => Ok(true),
        Err(_) => {
            Ok(false)
        }
    }
}

#[server(LoginExistsApi, "/api/login_exists")]
pub async fn login_exists(login: String) -> Result<bool,ServerFnError> {
    use server_imports::*;
    let state = expect_context::<AppState>();
    // let user : Result<Extension<UserData>,_> = extract().await;
    let pool = &state.pool;
    match User::get_by_login(&login, &pool) {
        Ok(_) => Ok(true),
        Err(_) => {
            Ok(false)
        }
    }
}

#[server(RegisterApi, "/api/register")]
pub async fn register(username:String,email:String,password:String) -> Result<(),ServerFnError> {
    use server_imports::*;
    let response = expect_context::<ResponseOptions>();
    let state = expect_context::<AppState>();
    let pool = &state.pool;
    match NewUser::new(username, email, password) {
        Ok(user) => {
            match User::add_user(user, &pool)
            {
                Ok(_) => Ok(()),
                Err(e) => {
                    response.set_status(StatusCode::BAD_REQUEST);
                    let msg = format!("Failed to register user: {}", e);
                    Err(ServerFnError::Response(msg))
                }
            }
        },
        Err(e) => {
            response.set_status(StatusCode::BAD_REQUEST);
            let msg = format!("Failed to register user: {}", e);
            Err(ServerFnError::Response(msg))
        }
    }
}