use http::HeaderMap;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::server_fn::codec::PostUrl;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(not(target_arch = "wasm32"))]
mod server_imports {
    pub use crate::db::models::user::{NewUser, User, UserLogin};
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

#[derive(Serialize, Deserialize, Debug)]
struct GoogleCBQuery {
    foo: Option<String>,
    bar: Option<String>,
}

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
    let RawQuery(raw_query): RawQuery = extract().await?;
    if let Some(raw_query) = &raw_query {
        let raw_params: HashMap<String, String> = form_urlencoded::parse(raw_query.as_bytes())
            .into_owned()
            .collect();
        log!("Received rawQuery = {:?}", raw_params);
    }
    log!("Received clientId = {}", clientId);
    log!("Received client_id = {}", client_id);
    log!("Received select_by = {}", select_by);
    log!("Received g_csrf_token = {}", g_csrf_token);
    log!("Received credential = {}", credential);
    let oauth_client = AsyncClient::new(clientId);
    let payload = oauth_client
        .validate_id_token(credential)
        .await
        .expect("Could not validate payload");
    log! {"{payload:?}"};
    //let Query(params): Query<GoogleCBQuery> = extract().await?;
    //log!("Received query = {:?}", params);
    return Ok(()); // TODO: Implement Google login callback
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
