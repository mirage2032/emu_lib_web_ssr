use axum_extra::extract::cookie::{Cookie, SameSite};
use http::header::InvalidHeaderValue;
use http::HeaderValue;
use std::time::Duration;

pub trait IntoHeaderValue {
    fn into_header_value(self) -> Result<HeaderValue, InvalidHeaderValue>;
}

impl IntoHeaderValue for Cookie<'_> {
    fn into_header_value(self) -> Result<HeaderValue, InvalidHeaderValue> {
        HeaderValue::from_str(&self.to_string())
    }
}

pub trait AppCookie<'a> {
    fn new_app_cookie(name: &'a str, value: &'a str, duration: Duration) -> Cookie<'a> {
        let cookie = Cookie::build((name, value))
            .http_only(true)
            .same_site(SameSite::Lax)
            .path("/")
            .secure(true)
            .expires(time::OffsetDateTime::now_utc() + duration)
            .build();
        cookie
    }
    fn expired_cookie(name: &'a str) -> Cookie<'a> {
        let cookie = Cookie::build((name, ""))
            .http_only(true)
            .same_site(SameSite::Lax)
            .path("/")
            .secure(true)
            .expires(time::OffsetDateTime::now_utc() - time::Duration::days(1))
            .build();
        cookie
    }
}

impl<'a> AppCookie<'a> for Cookie<'a> {}
