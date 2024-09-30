use app::db::models::session::Session;
use app::db::models::user::{User, UserData};
use app::db::AppState;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use axum_extra::extract::CookieJar;

pub async fn auth_middleware(
    State(app_state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let pool = app_state.pool;
    let jar = CookieJar::from_headers(req.headers());
    if let Some(cookie) = jar.get("session_token") {
        if let Ok(session) = Session::get_by_token(cookie.value(), &pool) {
            if !session.is_expired() {
                if let Ok(user) = User::get_by_id(session.user_id, &pool) {
                    let user_data: UserData = user.into();
                    req.extensions_mut().insert(user_data);
                }
            }
        }
    };
    Ok(next.run(req).await)
}
