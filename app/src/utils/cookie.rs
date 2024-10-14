pub struct AppCookie{}
pub enum CookieKey<'a>{
    Session,
    Other(&'a str)
}

impl<'a> AsRef<str> for CookieKey<'a>{
    fn as_ref(&self) -> &'a str {
        match self {
            CookieKey::Session=>"session_token",
            CookieKey::Other(key)=>*key
        }
    }
}

#[cfg(feature = "ssr")]
impl AppCookie {
    fn new_cookie<'a>(key:&'a CookieKey,value: &'a str, duration: time::Duration) -> axum_extra::extract::cookie::Cookie<'a> {
        let cookie = axum_extra::extract::cookie::Cookie::build((key.as_ref(), value))
            .same_site(axum_extra::extract::cookie::SameSite::Lax)
            .path("/")
            .secure(false)
            .expires(time::OffsetDateTime::now_utc() + duration)
            .build();
        cookie
    }
    pub fn set(key:&CookieKey,value: &str, duration:time::Duration, response: &leptos_axum::ResponseOptions) -> Result<(), http::header::InvalidHeaderValue> {
        use http::{header::SET_COOKIE,HeaderValue};
        let cookie = Self::new_cookie(
            key,
            value,
            duration,
        );
        response.append_header(SET_COOKIE, HeaderValue::from_str(&cookie.to_string())?);
        Ok(())
    }

    pub fn remove(key:&CookieKey,response: &leptos_axum::ResponseOptions) -> Result<(),http::header::InvalidHeaderValue>{
        use http::{header::SET_COOKIE,HeaderValue};
        let cookie = Self::new_cookie(
            key,
            "",
            time::Duration::seconds(-60*24)
        );
        response.append_header(SET_COOKIE, HeaderValue::from_str(&cookie.to_string())?);
        Ok(())
    }
}
#[cfg(not(feature = "ssr"))]
impl AppCookie {
    pub fn set(key:&CookieKey,value: &str, duration:std::time::Duration) {
        let options = wasm_cookies::cookies::CookieOptions{
            path:Some("/"),
            domain:None,
            expires:None,
            secure:false,
            same_site:wasm_cookies::cookies::SameSite::Lax
        }.expires_after(duration);
        wasm_cookies::set(key.as_ref(), value, &options);
    }

    pub fn remove(key:&CookieKey){
        wasm_cookies::delete(key.as_ref())
    }
}