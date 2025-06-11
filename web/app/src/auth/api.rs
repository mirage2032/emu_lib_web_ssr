use crate::db::models::user::{User, UserData};
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

const AUTH_TIMEOUT: time::Duration = time::Duration::seconds(60 * 60 * 24);

#[cfg(not(target_arch = "wasm32"))]
mod server_imports {
    pub use crate::db::models::user::{NewUser, User, UserLogin};
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
    let user_login = UserLogin::new(login, password);
    match user_login.get_user(&pool) {
        Ok(user) => {
            if let Ok(session) = user.authenticate(&pool, AUTH_TIMEOUT)
            {
                cookie::server::set(&CookieKey::Session, &session.token, AUTH_TIMEOUT, &response)?;
                Ok(())
            }else {
                Err(ServerFnError::ServerError("Failed to authenticate user".to_string()))
            }
        }
        Err(e) => {
            cookie::server::remove(&CookieKey::Session, &response)?;
            response.set_status(StatusCode::UNAUTHORIZED);
            let msg = format!("Failed to login user: {}", e);
            Err(ServerFnError::Response(msg))
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_google_user(token: &str) -> Option<User> {
    use server_imports::*;
    let state = expect_context::<AppState>();
    let pool = state.pool;
    User::get_by_google_oauth(token, &pool).ok()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_github_user(token: &str) -> Option<User> {
    use server_imports::*;
    let state = expect_context::<AppState>();
    let pool = state.pool;
    User::get_by_github_oauth(token, &pool).ok()
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn user_auth(user:User,duration:time::Duration) -> Result<(), ServerFnError> {
    use server_imports::*;
    let state = expect_context::<AppState>();
    if let Ok(session) = user
        .authenticate(&state.pool, duration) {
        cookie::server::set(
            &CookieKey::Session,
            &session.token,
            duration,
            &expect_context::<ResponseOptions>(),
        )?;
        Ok(())
    } else {
        Err(ServerFnError::ServerError("Failed to authenticate user".to_string()))
    }
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
    let state = expect_context::<AppState>();
    let payload = oauth_client
        .validate_id_token(credential)
        .await
        .expect("Could not validate payload");
    if let Some(user) = get_google_user(&payload.sub) {
        user_auth(user, AUTH_TIMEOUT).await
    } else {
        //create random password
        let random_password: String = (0..16).map(|_| rand::random::<char>()).collect();
        if let (Some(name),Some(email)) = (payload.name,payload.email) {
            let name = name.split(" ").next();
            let name = match name {
                Some(n) => n.to_string(),
                None => {
                    return Err(ServerFnError::Response(
                        "Could not get name for Google Auth registration".to_string(),
                    ))
                }
            };
            let new_user = NewUser::new(
                name,
                email.clone(),
                random_password.clone(),
                Some(payload.sub.clone()),
                None,
            );
            if let Ok(user) = new_user {
                if let Ok(user) = User::add_user(user, &state.pool) {
                    user_auth(user, time::Duration::seconds(60 * 60 * 24)).await?;
                    Ok(())
                } else {
                    Err(ServerFnError::Response(
                        "Failed to register user with Google Auth".to_string(),
                    ))
                }
            } else {
                Err(ServerFnError::Response(
                    "Failed to create new user from Google Auth data".to_string(),
                ))
            }
        } else {
            Err(ServerFnError::Response(
                "Could not get name for Google Auth registration".to_string(),
            ))
        }
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
    use std::env;
    use server_imports::*;
    let state = expect_context::<AppState>();
    let github_secret = env::var("GITHUB_CLIENT_SECRET").expect("No GITHUB_CLIENT_SECRET set");
    let client = github_oauth_client().set_client_secret(ClientSecret::new(github_secret));
    let token_result = client
        .exchange_code(AuthorizationCode::new(code))
        .request_async(&state.reqwest_client)
        .await?;
    let access_token = token_result.access_token().secret();
    let res_user = state
        .reqwest_client
        .get("https://api.github.com/user")
        .bearer_auth(access_token)
        .header("User-Agent", "Leptos App")
        .send()
        .await?;
    let user: GithubUser = res_user.json().await?;

    if let Some(user) = get_github_user(&user.id.to_string()) {
        user_auth(user, AUTH_TIMEOUT).await?;
        return Ok(());
    }
    else{
        let password: String = (0..16).map(|_| rand::random::<char>()).collect();
        if let Some(email) = user.email{
            let new_user = NewUser::new(
                user.login.clone(),
                email.clone(),
                password.clone(),
                None,
                Some(user.id.to_string()),
            );
            if let Ok(user) = new_user {
                if let Ok(user) = User::add_user(user, &state.pool) {
                    user_auth(user, AUTH_TIMEOUT).await?;
                    return Ok(());
                } else {
                    return Err(ServerFnError::Response(
                        "Failed to register user with GitHub Auth".to_string(),
                    ));
                }
            } else {
                return Err(ServerFnError::Response(
                    "Failed to create new user from GitHub Auth data".to_string(),
                ));
            }
        }
        else{
            return Err(ServerFnError::Response(
                "Could not get email for GitHub Auth registration".to_string(),
            ));
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
    match NewUser::new(username, email, password, None, None) {
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
