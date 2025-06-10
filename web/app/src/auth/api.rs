use crate::db::models::user::UserData;
use crate::utils::ccompiler::{CompileData, CompilerError};
use http::HeaderMap;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::server_fn::codec::PostUrl;
use oauth2::basic::*;
use oauth2::{
    Client, ClientSecret, EndpointNotSet, EndpointSet, StandardRevocableToken, TokenResponse,
};
use serde::{Deserialize, Serialize};
use server_fn::codec::JsonEncoding;
use std::collections::HashMap;
use thiserror::Error;

#[cfg(not(target_arch = "wasm32"))]
mod server_imports {
    pub use crate::db::models::user::{EmailNoPasswordLogin, NewUser, User, UserLogin};
    pub use crate::db::AppState;
    pub use crate::utils::cookie::{self, CookieKey};
    pub use axum::extract::RawQuery;
    pub use axum::Extension;
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
    use server_imports::*;
    let oauth_client = google_oauth::AsyncClient::new(clientId);
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
                let name = name.split(" ").next();
                let name = match name {
                    Some(n) => n.to_string(),
                    None => {
                        return Err(ServerFnError::Response(
                            "Could not get name for Google Auth registration".to_string(),
                        ))
                    }
                };
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

fn github_oauth_client() -> Client<
    BasicErrorResponse,
    BasicTokenResponse,
    BasicTokenIntrospectionResponse,
    StandardRevocableToken,
    BasicRevocationErrorResponse,
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet,
> {
    use oauth2::{basic::BasicClient, AuthUrl, ClientId, RedirectUrl, TokenUrl};
    BasicClient::new(ClientId::new("Ov23liaBVBCExpfVdE5h".to_string()))
        .set_redirect_uri(
            RedirectUrl::new("http://localhost:3000/auth/github_login_callback".to_string())
                .expect("Could not set redirect URI"),
        )
        .set_auth_uri(
            AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
                .expect("Could not set redirect URI"),
        )
        .set_token_uri(
            TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
                .expect("Could not set redirect URI"),
        )
}

pub fn github_auth_url() -> String {
    use oauth2::Scope;
    let client = github_oauth_client();
    let (auth_url, _csrf_token) = client
        .authorize_url(oauth2::CsrfToken::new_random)
        .add_scope(Scope::new("user:email".to_string()))
        .url();
    auth_url.to_string()
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Serialize, Deserialize, Debug)]
struct GithubEmail {
    email: String,
    primary: bool,
    verified: bool,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Serialize, Deserialize, Debug)]
struct GithubUser {
    login: String, // This is the username
    id: u64,
    name: Option<String>,
    email: Option<String>,
}

#[allow(non_snake_case)]
#[server(GithubLoginCallbackApi, endpoint = "/github_login_callback")]
pub async fn github_login_callback(
    code: String,
    state: Option<String>,
) -> Result<(), ServerFnError> {
    use oauth2::AuthorizationCode;
    use server_imports::*;
    let state = expect_context::<AppState>();
    let client = github_oauth_client().set_client_secret(ClientSecret::new(
        "0d5294fcb87fcf1716af6ff91c8347c8021fdf57".to_string(),
    ));
    let token_result = client
        .exchange_code(AuthorizationCode::new(code))
        .request_async(&state.reqwest_client)
        .await?;
    let access_token = token_result.access_token().secret();
    let res_email = state
        .reqwest_client
        .get("https://api.github.com/emails")
        .bearer_auth(access_token)
        .header("User-Agent", "Leptos App")
        .send()
        .await?;
    let mut emails: Vec<GithubEmail> = res_email.json().await?;
    emails.retain(|email| email.verified);
    emails.sort_by(|a, b| b.primary.cmp(&a.primary));
    //check email_exists() on each until one is found
    for email in emails {
        if email_exists(email.email.clone()).await? {
            unsafe { login_email_no_password(email.email.clone()).await? }
        } else {
            //create random password
            let random_password: String = (0..16).map(|_| rand::random::<char>()).collect();
            let res_user = state
                .reqwest_client
                .get("https://api.github.com/user")
                .bearer_auth(access_token)
                .header("User-Agent", "Leptos App")
                .send()
                .await?;
            let user: GithubUser = res_user.json().await?;
            if let Ok(()) = register(
                user.login,
                email.email.clone(),
                random_password.clone(),
            )
            .await
            {
                login(email.email, random_password).await?
            } else {
                return Err(ServerFnError::Response(format!(
                    "Failed to register user with email: {}",
                    email.email
                )));
            }
        }
    }
    Err(ServerFnError::Response(
        "Could not login via github".to_string(),
    ))
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

#[derive(Clone, Error, Debug, Serialize, Deserialize)]
pub enum UserDataError {
    #[error("Unauthenticated")]
    Unauthenticated,
    #[error("Server error: {0}")]
    ServerError(String),
}

impl FromServerFnError for UserDataError {
    type Encoder = JsonEncoding;
    fn from_server_fn_error(value: ServerFnErrorErr) -> Self {
        UserDataError::ServerError(value.to_string())
    }
}
#[server(GetUserData, endpoint = "/userdata")]
pub async fn userdata() -> Result<UserData, UserDataError> {
    use server_imports::*;
    let userdata: Result<Extension<UserData>, _> = extract().await;
    if let Ok(userdata) = userdata {
        Ok(userdata.0.clone())
    } else {
        log!("User is not authenticated");
        Err(UserDataError::Unauthenticated)
    }
}
