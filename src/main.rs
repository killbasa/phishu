mod config;
mod constants;
mod pages;
mod utils;
mod youtube;

use std::{
    net::{Ipv4Addr, SocketAddr},
    str::FromStr,
};

use axum::{
    Router,
    http::{HeaderMap, StatusCode},
    response::Redirect,
    routing::get,
};
use axum_extra::{TypedHeader, headers::UserAgent};
use config::CONFIG;
use pages::{PageContext, Pages, Render};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();

    let host = Ipv4Addr::from_str(&CONFIG.server.host).expect("invalid host");
    let socket = SocketAddr::from((host, CONFIG.server.port));
    let listener = tokio::net::TcpListener::bind(&socket).await.unwrap();

    tracing::info!("listening on http://{}", &socket);

    axum::serve(
        listener, //
        app().into_make_service(),
    )
    .await
    .expect("server error");
}

fn app() -> Router {
    Router::new() //
        .fallback(fallback)
        // Routes
        .route("/", get(get_root))
        .route("/info", get(get_info))
        .route("/upcoming", get(get_upcoming))
        .route("/lastseen", get(get_lastseen))
        // Redirects
        .route("/git", get(Redirect::permanent(&CONFIG.git_repo)))
        .route("/website", get(Redirect::temporary("https://aegis-l.ink/talent/triggerphish")))
        .route(
            "/store",
            get(Redirect::temporary(
                "https://merch.kawaentertainment.com/en-ca/collections/trigg3rph-h",
            )),
        )
        .route("/youtube", get(Redirect::permanent("https://www.youtube.com/@TRiGGERPHiSH")))
        .route("/twitter", get(Redirect::permanent("https://twitter.com/TRiGGERPH1SH")))
        .route("/discord", get(Redirect::permanent("https://discord.com/invite/4GHZZMm4Sp")))
        .route("/twitch", get(Redirect::permanent("https://www.twitch.tv/triggerph1sh")))
        .route("/tiktok", get(Redirect::permanent("https://www.tiktok.com/@triggerphish_al")))
        .route("/reddit", get(Redirect::permanent("https://www.reddit.com/user/TriggerPh1sh/")))
}

fn render(user_agent: UserAgent, page: Pages) -> impl axum::response::IntoResponse {
    let mut headers = HeaderMap::new();
    let ctx = PageContext { host: CONFIG.public_host.clone() };

    let content = if user_agent.to_string().starts_with("curl") {
        headers.insert("Content-Type", "text/plain".parse().unwrap());
        page.render_term(ctx)
    } else {
        headers.insert("Content-Type", "text/html".parse().unwrap());
        page.render_html(ctx)
    };

    (StatusCode::OK, headers, content)
}

// 404 handler
async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, format!("no route {}", uri))
}

// GET /
async fn get_root(
    TypedHeader(user_agent): TypedHeader<UserAgent>,
) -> impl axum::response::IntoResponse {
    render(user_agent, Pages::Root)
}

// GET /info
async fn get_info(
    TypedHeader(user_agent): TypedHeader<UserAgent>,
) -> impl axum::response::IntoResponse {
    render(user_agent, Pages::Info)
}

// GET /upcoming
async fn get_upcoming(
    TypedHeader(user_agent): TypedHeader<UserAgent>,
) -> impl axum::response::IntoResponse {
    render(user_agent, Pages::Upcoming)
}

// GET /lastseen
async fn get_lastseen(
    TypedHeader(user_agent): TypedHeader<UserAgent>,
) -> impl axum::response::IntoResponse {
    render(user_agent, Pages::LastSeen)
}
