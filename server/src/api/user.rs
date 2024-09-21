use crate::api::{DbPool, StatusMsgResponse};
use crate::middleware::UserData;
use crate::models::user::{NewUser, User, UserLogin};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json, Router};
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;
use axum_macros::debug_handler;
use leptos::LeptosOptions;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

#[debug_handler]
async fn register(
    Extension(pool): Extension<DbPool>,
    Json(body): Json<RegisterRequest>,
) -> impl IntoResponse {
    let new_user = match NewUser::new(body.username, body.email, body.password) {
        Ok(user) => user,
        Err(e) => {
            let response = StatusMsgResponse {
                success: false,
                message: format!("Failed to register user: {}", e),
            };
            return (StatusCode::BAD_REQUEST, Json(response));
        }
    };
    match User::add_user(new_user, &pool) {
        Ok(user) => {
            let response = StatusMsgResponse {
                success: true,
                message: format!("User {} registered successfully!", user.username),
            };
            (StatusCode::CREATED, Json(response))
        }
        Err(e) => {
            let response = StatusMsgResponse {
                success: false,
                message: format!("Failed to register user: {}", e),
            };
            (StatusCode::BAD_REQUEST, Json(response))
        }
    }
}

#[derive(Serialize, Deserialize)]
struct LoginRequest {
    pub login: String,
    pub password: String,
}
#[debug_handler]
async fn login(
    // jar: CookieJar,
    Extension(pool): Extension<DbPool>,
    Json(body): Json<LoginRequest>,
) -> impl IntoResponse {
    let user = match body.login.contains('@') {
        true => UserLogin::new_with_email(body.login, body.password),
        false => UserLogin::new_with_username(body.login, body.password),
    };
    let duration = std::time::Duration::from_secs(3600 * 24);
    match user.authenticate(&pool, duration) {
        Ok((user, session)) => {
            let mut jar = CookieJar::new();
            let cookie = Cookie::build(("session_token", format!("{}", session.token)))
                .http_only(true)
                .build();
            jar = jar.add(cookie);

            let response = StatusMsgResponse {
                success: true,
                message: format!("User {} logged in successfully!", user.username),
            };
            (StatusCode::OK, Some(jar), Json(response))
        }
        Err(e) => {
            let response = StatusMsgResponse {
                success: false,
                message: format!("Failed to login: {}", e),
            };
            (StatusCode::UNAUTHORIZED, None, Json(response))
        }
    }
}

#[debug_handler]
async fn logout(
    Extension(pool): Extension<DbPool>,
    user_data: Option<Extension<UserData>>,
    // jar: CookieJar,
) -> impl IntoResponse {
    if user_data.is_none() {
        let response = StatusMsgResponse {
            success: false,
            message: "No user logged in!".to_string(),
        };
        return (StatusCode::OK, None, Json(response));
    }
    let mut jar = CookieJar::new();
    let cookie = Cookie::build(("session_token", ""))
        .http_only(true)
        .expires(time::OffsetDateTime::now_utc() - time::Duration::days(1))
        .build();
    jar = jar.add(cookie);
    let response = StatusMsgResponse {
        success: true,
        message: "Logged out successfully!".to_string(),
    };
    (StatusCode::OK, Some(jar), Json(response))
}
pub fn user_routes() -> Router<LeptosOptions> {
    Router::new()
        .route("/register", axum::routing::post(register))
        .route("/login", axum::routing::post(login))
        .route("/logout", axum::routing::post(logout))
}
