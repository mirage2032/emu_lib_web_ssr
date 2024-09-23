use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use axum_extra::extract::CookieJar;
use app::db::DbPool;
use app::db::models::session::Session;
use app::db::models::user::User;

pub type UserData = (Session, User);
pub async fn auth_middleware(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let pool = req.extensions().get::<DbPool>().unwrap();
    let jar = CookieJar::from_headers(req.headers());
    if let Some(cookie) = jar.get("session_token") {
        if let Ok(session) = Session::get_by_token(cookie.value(), &pool) {
            if !session.is_expired() {
                if let Ok(user) = User::get_by_id(session.user_id, &pool) {
                    let user_data: UserData = (session, user);
                    req.extensions_mut().insert(user_data);
                }
            }
        }
    };
    Ok(next.run(req).await)
}
