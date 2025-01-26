pub enum CookieKey<'a> {
    Session,
    Other(&'a str),
}

impl<'a> CookieKey<'a> {
    fn as_str(&'a self) -> &'a str {
        match self {
            CookieKey::Session => "session_token",
            CookieKey::Other(key) => *key,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub mod server {
    use super::CookieKey;
    use axum_extra::extract::cookie::{Cookie, SameSite};
    use axum_extra::extract::CookieJar;
    use http::{header::SET_COOKIE, HeaderMap, HeaderValue};

    fn new_cookie<'a>(key: &'a CookieKey, value: &'a str, duration: time::Duration) -> Cookie<'a> {
        let cookie = Cookie::build((key.as_str(), value))
            .same_site(SameSite::Lax)
            .path("/")
            .secure(false)
            .expires(time::OffsetDateTime::now_utc() + duration)
            .build();
        cookie
    }
    pub fn set(
        key: &CookieKey,
        value: &str,
        duration: time::Duration,
        response: &leptos_axum::ResponseOptions,
    ) -> Result<(), http::header::InvalidHeaderValue> {
        let cookie = new_cookie(key, value, duration);
        response.append_header(SET_COOKIE, HeaderValue::from_str(&cookie.to_string())?);
        Ok(())
    }

    pub fn get<'a>(key: &CookieKey<'a>, headers: &HeaderMap) -> Option<String> {
        let jar = CookieJar::from_headers(&headers);
        if let Some(cookie) = jar.get(key.as_str()) {
            Some(cookie.value().to_string())
        } else {
            None
        }
    }

    pub fn remove(
        key: &CookieKey,
        response: &leptos_axum::ResponseOptions,
    ) -> Result<(), http::header::InvalidHeaderValue> {
        let cookie = new_cookie(key, "", time::Duration::seconds(-60 * 24));
        response.append_header(SET_COOKIE, HeaderValue::from_str(&cookie.to_string())?);
        Ok(())
    }
}
// pub mod wasm {
//     use super::CookieKey;
//     use wasm_cookies::cookies::CookieOptions;
//     use wasm_cookies::cookies::SameSite;
//
//     #[cfg(target_arch = "wasm32")]
//     pub fn set(key: &CookieKey, value: &str, duration: time::Duration) {
//         let options = CookieOptions {
//             path: Some("/"),
//             domain: None,
//             expires: None,
//             secure: false,
//             same_site: SameSite::Lax,
//         }
//             .expires_after(duration);
//         wasm_cookies::set(key.as_str(), value, &options);
//     }
//     #[cfg(not(target_arch = "wasm32"))]
//     pub fn set(_key: &CookieKey, _value: &str, _duration: time::Duration) {}
//
//     #[cfg(target_arch = "wasm32")]
//     pub fn get(key: &CookieKey) -> Option<String> {
//         wasm_cookies::get_raw(key.as_str())
//     }
//
//     #[cfg(not(target_arch = "wasm32"))]
//     pub fn get(key: &CookieKey) -> Option<String> {
//         None
//     }
//     #[cfg(target_arch = "wasm32")]
//     pub fn remove(key: &CookieKey) {
//         wasm_cookies::delete(key.as_str());
//     }
//     #[cfg(not(target_arch = "wasm32"))]
//     pub fn remove(_key: &CookieKey) {}
// }
