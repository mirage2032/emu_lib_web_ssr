use http::header::InvalidHeaderValue;
use thiserror::Error;

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

#[derive(Error, Debug)]
pub enum SetCookieError {
    #[error("Missing ResponseOptions in context")]
    MissingResponseOptions,
    #[error("Invalid header value")]
    InvalidHeaderValue(InvalidHeaderValue),
}
#[derive(Error, Debug)]
pub enum RemoveCookieError {
    #[error("Missing ResponseOptions in context")]
    MissingResponseOptions,
    #[error("Invalid header value")]
    InvalidHeaderValue(InvalidHeaderValue),
}

#[derive(Error, Debug)]
pub enum GetCookieError {
    #[error("Missing HeaderMap in context")]
    MissingHeaderMap,
    #[error("Cookie not found")]
    NotFound,
}
#[cfg(not(target_arch = "wasm32"))]
pub mod cookieops {
    use super::{CookieKey, GetCookieError, RemoveCookieError, SetCookieError};
    use axum_extra::extract::cookie::{Cookie, SameSite};
    use axum_extra::extract::CookieJar;
    use http::{header::SET_COOKIE, HeaderMap, HeaderValue};
    use leptos::prelude::{use_context};
    use leptos_axum::{ResponseOptions};

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
    ) -> Result<(), SetCookieError> {
        if let Some(response_options) = use_context::<ResponseOptions>() {
            let cookie = new_cookie(key, value, duration);
            response_options.append_header(SET_COOKIE, HeaderValue::from_str(&cookie.to_string()).map_err(|err|SetCookieError::InvalidHeaderValue(err))?);
            Ok(())
        }
        else {
            Err(SetCookieError::MissingResponseOptions)
        }
    }

    pub fn get(key: &CookieKey, headers: &HeaderMap) -> Result<String, GetCookieError> {
        let jar = CookieJar::from_headers(&headers);
        if let Some(cookie) = jar.get(key.as_str()) {
            Ok(cookie.value().to_string())
        } else {
            Err(GetCookieError::NotFound)
        }
    }

    pub fn remove(key: &CookieKey) -> Result<(), RemoveCookieError> {
        if let Some(response_options) = use_context::<ResponseOptions>() {
            let cookie = new_cookie(key, "", time::Duration::seconds(-60 * 24));
            response_options.append_header(SET_COOKIE, HeaderValue::from_str(&cookie.to_string()).map_err(|err|RemoveCookieError::InvalidHeaderValue(err))?);
            Ok(())
        }
        else{
            Err(RemoveCookieError::MissingResponseOptions)
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub mod cookieops {
    use http::HeaderMap;
    use super::{CookieKey, GetCookieError, RemoveCookieError, SetCookieError};
    use wasm_cookies::cookies::CookieOptions;
    use wasm_cookies::cookies::SameSite;

    pub fn set(
        key: &CookieKey,
        value: &str,
        duration: time::Duration,
    ) -> Result<(), SetCookieError> {
        let options = CookieOptions {
            path: Some("/"),
            domain: None,
            expires: None,
            secure: false,
            same_site: SameSite::Lax,
        }
        .expires_after(duration);
        wasm_cookies::set(key.as_str(), value, &options);
    }
    pub fn get(key: &crate::utils::cookie2::CookieKey, headers: &HeaderMap) -> Result<String, crate::utils::cookie2::GetCookieError> {
        Ok(
            wasm_cookies::get_raw(key.as_str())
        )
    }
    pub fn remove(key: &crate::utils::cookie2::CookieKey) -> Result<(), crate::utils::cookie2::RemoveCookieError> {
        wasm_cookies::delete(key.as_str());
        Ok(())
    }
}
