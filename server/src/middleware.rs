use app::db::models::session::Session;
use app::db::models::user::{User, UserData};
use app::db::AppState;
use app::utils::cookie;
use app::utils::cookie::CookieKey;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;

pub async fn auth_middleware(
    State(app_state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let pool = app_state.pool;
    if let Ok(session_token) = cookie::cookieops::get(&CookieKey::Session) {
        if let Ok(session) = Session::get_by_token(&session_token, &pool) {
            if !session.is_expired() {
                if let Ok(user) = User::get_by_id(session.user_id, &pool) {
                    let user_data: UserData = user.into();
                    req.extensions_mut().insert(user_data);
                }
            }
        }
    }
    Ok(next.run(req).await)
}
