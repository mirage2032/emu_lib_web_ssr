
const COOKIE_NAME:&str = "session_cookie";
pub struct SessionCookie{}

#[cfg(feature = "ssr")]
impl SessionCookie {
    fn new_cookie<'a>(value: &'a str, duration: time::Duration) -> axum_extra::extract::cookie::Cookie<'a> {
        let cookie = axum_extra::extract::cookie::Cookie::build((COOKIE_NAME, value))
            .http_only(true)
            .same_site(axum_extra::extract::cookie::SameSite::Lax)
            .path("/")
            .secure(true)
            .expires(time::OffsetDateTime::now_utc() + duration)
            .build();
        cookie
    }
    pub fn set(token: &str, duration:time::Duration, response: &leptos_axum::ResponseOptions) -> Result<(), http::header::InvalidHeaderValue> {
        use http::{header::SET_COOKIE,HeaderValue};
        let cookie = Self::new_cookie(
            token,
            duration,
        );
        response.append_header(SET_COOKIE, HeaderValue::from_str(&cookie.to_string())?);
        Ok(())
    }

    pub fn remove(response: &leptos_axum::ResponseOptions) -> Result<(),http::header::InvalidHeaderValue>{
        use http::{header::SET_COOKIE,HeaderValue};
        let cookie = Self::new_cookie(
            "",
            time::Duration::seconds(-60*24)
        );
        response.append_header(SET_COOKIE, HeaderValue::from_str(&cookie.to_string())?);
        Ok(())
    }
}
#[cfg(not(feature = "ssr"))]
impl SessionCookie {
    pub fn set(token: &str, duration:std::time::Duration) {
        let options = wasm_cookies::cookies::CookieOptions{
            path:Some("/"),
            domain:None,
            expires:None,
            secure:true,
            same_site:wasm_cookies::cookies::SameSite::Lax
        }.expires_after(duration);
        wasm_cookies::set(COOKIE_NAME, token, &options);
    }

    fn remove(){
        wasm_cookies::delete(COOKIE_NAME)
    }
}