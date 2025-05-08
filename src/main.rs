mod config;
mod constants;
mod utils;
mod youtube;

use std::{
    fs,
    net::{Ipv4Addr, SocketAddr},
    str::FromStr,
    time::Duration,
};

use axum::{
    Router,
    http::{HeaderMap, StatusCode},
    response::Redirect,
    routing::get,
};
use axum_extra::{TypedHeader, headers::UserAgent};
use config::CONFIG;
use once_cell::sync::Lazy;
use tower::ServiceBuilder;
use tower_http::{
    LatencyUnit,
    timeout::TimeoutLayer,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use utils::root_banner;
use youtube::get_channel_from_api;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let host = Ipv4Addr::from_str(&CONFIG.server.host).expect("invalid host");
    let addr = SocketAddr::from((host, CONFIG.server.port));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    tracing::event!(tracing::Level::INFO, "listening on http://{}", &addr);

    axum::serve(
        listener, //
        app().into_make_service(),
    )
    .await
    .expect("server error");
}

fn app() -> Router {
    let middleware = ServiceBuilder::new()
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_response(DefaultOnResponse::new().latency_unit(LatencyUnit::Micros)),
        )
        .layer(TimeoutLayer::new(Duration::from_secs(10)));

    Router::new() //
        .fallback(fallback)
        .layer(middleware)
        // Routes
        .route("/", get(get_root))
        .route("/info", get(get_info))
        .route("/upcoming", get(get_upcoming))
        .route("/lastseen", get(get_lastseen))
        // Redirects
        .route("/git", get(Redirect::permanent(&CONFIG.git_repo)))
        .route("/website", get(Redirect::temporary("https://aegis-l.ink/talent/triggerphish")))
        .route("/youtube", get(Redirect::permanent("https://www.youtube.com/@TRiGGERPHiSH")))
        .route("/twitter", get(Redirect::permanent("https://twitter.com/TRiGGERPH1SH")))
        .route("/discord", get(Redirect::permanent("https://discord.com/invite/4GHZZMm4Sp")))
        .route("/twitch", get(Redirect::permanent("https://www.twitch.tv/triggerph1sh")))
        .route("/tiktok", get(Redirect::permanent("https://www.tiktok.com/@triggerphish_al")))
        .route("/reddit", get(Redirect::permanent("https://www.reddit.com/user/TriggerPh1sh/")))
}

// 404 handler
async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, format!("no route {}", uri))
}

// GET /
static ROOT_BANNER: Lazy<String> = Lazy::new(|| root_banner(&CONFIG.public_host));
static ROOT_HTML: Lazy<String> = Lazy::new(|| fs::read_to_string("assets/index.html").unwrap());

async fn get_root(
    TypedHeader(user_agent): TypedHeader<UserAgent>,
) -> impl axum::response::IntoResponse {
    let mut headers = HeaderMap::new();

    if user_agent.to_string().starts_with("curl") {
        headers.insert("Content-Type", "text/plain".parse().unwrap());
        (StatusCode::OK, headers, ROOT_BANNER.clone())
    } else {
        headers.insert("Content-Type", "text/html".parse().unwrap());
        (StatusCode::OK, headers, ROOT_HTML.clone())
    }
}

// GET /info
async fn get_info() -> impl axum::response::IntoResponse {
    get_channel_from_api()
}

// GET /upcoming
async fn get_upcoming() -> impl axum::response::IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "not implemented")
}

// GET /lastseen
async fn get_lastseen() -> impl axum::response::IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "not implemented")
}
