#![recursion_limit = "512"]

use app::db::{establish_connection, AppState};
use app::*;
use axum::http::{HeaderValue, Method};
use axum::middleware::from_fn_with_state;
use axum::Router;
use fileserv::file_and_error_handler;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use reqwest::Client;
use tower_http::compression::predicate::{NotForContentType, SizeAbove};
use tower_http::compression::{CompressionLayer, Predicate};
use tower_http::cors::{Any, CorsLayer};
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::CompressionLevel;
// mod api;
pub mod fileserv;
mod middleware;

#[tokio::main]
async fn main() {
    let public_url = std::env::var("PUBLIC_URL").expect("PUBLIC_URL");
    let cors = CorsLayer::new()
        .allow_origin(format!("http://{public_url}").parse::<HeaderValue>().unwrap())
        .allow_methods(Any)
        .allow_headers(Any);
    //.allow_methods([Method::GET, Method::POST])
    //.allow_headers([CONTENT_TYPE]);
    let coop_layer = SetResponseHeaderLayer::if_not_present(
        axum::http::header::HeaderName::from_static("cross-origin-opener-policy"),
        HeaderValue::from_static("same-origin-allow-popups"),
    );

    let coep_layer = SetResponseHeaderLayer::if_not_present(
        axum::http::header::HeaderName::from_static("cross-origin-embedder-policy"),
        HeaderValue::from_static("credentialless"),
    );
    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");
    let pool = establish_connection().await;
    let predicate = SizeAbove::new(1500) // files smaller than 1501 bytes are not compressed, since the MTU (Maximum Transmission Unit) of a TCP packet is 1500 bytes
        .and(NotForContentType::GRPC)
        .and(NotForContentType::IMAGES)
        // prevent compressing assets that are already statically compressed
        .and(NotForContentType::const_new("application/javascript"))
        .and(NotForContentType::const_new("application/wasm"))
        .and(NotForContentType::const_new("text/css"));
    // Setting R(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // build our application with a route
    let state = AppState {
        leptos_options: leptos_options.clone(),
        pool: pool.clone(),
        reqwest_client: Client::builder()
            .build()
            .expect("Could not create reqwest client"),
    };
    let state_clone = state.clone();
    let app = Router::new()
        // .leptos_routes(&leptos_options, routes, App)
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            move || provide_context(state_clone.clone()),
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler(shell))
        .layer(cors)
        //.layer(coop_layer)
        //.layer(coep_layer)
        .layer(from_fn_with_state(
            state.clone(),
            middleware::auth_middleware,
        ))
        .layer(
            CompressionLayer::new()
                .quality(CompressionLevel::Fastest)
                .compress_when(predicate),
        )
        .with_state(leptos_options);

    // run our app with hyperr
    // `axum::Server` is a re-export of `hyper::Server`
    log::info!("listening on http://{}", &addr);
    match tokio::net::TcpListener::bind(addr).await{
        Ok(listener) => {
            axum::serve(listener, app.into_make_service())
                .await
                .unwrap();
        }
        Err(e) => {
            log::error!("Failed to bind to address {}: {}", addr, e);
            std::process::exit(1);
        }
    }
}
