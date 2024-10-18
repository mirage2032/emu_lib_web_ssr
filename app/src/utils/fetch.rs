use leptos::prelude::expect_context;
use serde::de::DeserializeOwned;
use serde::Serialize;

#[cfg(not(feature = "ssr"))]
pub fn fetch_object<T>(
    path: &str,
) -> impl std::future::Future<Output = Result<T, String>> + Send + '_
where
    T: Serialize + DeserializeOwned,
{
    use leptos::prelude::on_cleanup;
    use send_wrapper::SendWrapper;

    SendWrapper::new(async move {
        let abort_controller = SendWrapper::new(leptos::web_sys::AbortController::new().ok());
        let abort_signal = abort_controller.as_ref().map(|a| a.signal());

        // abort in-flight requests if, e.g., we've navigated away from this page
        on_cleanup(move || {
            if let Some(abort_controller) = abort_controller.take() {
                abort_controller.abort()
            }
        });
        gloo_net::http::Request::get(path)
            .abort_signal(abort_signal.as_ref())
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json()
            .await
            .map_err(|e| e.to_string())
    })
}

#[cfg(feature = "ssr")]
pub async fn fetch_object<T>(path: &str) -> Result<T, String>
where
    T: Serialize + DeserializeOwned,
{
    pub use crate::db::AppState;
    let client = expect_context::<AppState>().reqwest_client;
    client
        .get(path)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}
