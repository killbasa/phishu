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
    http::{HeaderMap, StatusCode, header::CONTENT_TYPE},
    response::Redirect,
    routing::get,
};
use axum_extra::{TypedHeader, headers::UserAgent};
use config::CONFIG;
use constants::HTML_CSP;
use dotenv::dotenv;
use pages::{PageContext, Pages, Render};
use reqwest::header::CONTENT_SECURITY_POLICY;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt().with_max_level(CONFIG.server.log_level).init();

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
        .route("/youtube", get(Redirect::permanent(&CONFIG.vtuber.channel_url)))
        .route("/twitter", get(Redirect::permanent(&CONFIG.vtuber.socials.twitter)))
        .route("/discord", get(Redirect::permanent(&CONFIG.vtuber.socials.discord)))
        .route("/twitch", get(Redirect::permanent(&CONFIG.vtuber.socials.twitch)))
        .route("/tiktok", get(Redirect::permanent(&CONFIG.vtuber.socials.tiktok)))
        .route("/reddit", get(Redirect::permanent(&CONFIG.vtuber.socials.reddit)))
        .route("/website", get(Redirect::temporary(&CONFIG.vtuber.socials.website)))
        .route("/store", get(Redirect::temporary(&CONFIG.vtuber.socials.store)))
        // Assets
        .route("/favicon.ico", get(get_favicon))
    // .route("/csp-report", post(handle_cspreport))
}

async fn render(user_agent: UserAgent, page: Pages) -> impl axum::response::IntoResponse {
    let mut headers = HeaderMap::new();

    let ctx = PageContext {
        host: CONFIG.public_host.clone(),
        is_term: user_agent.to_string().starts_with("curl"),
    };

    let content = if ctx.is_term {
        headers.insert(CONTENT_TYPE, "text/plain".parse().unwrap());

        page.render_term(ctx).await
    } else {
        headers.insert(CONTENT_TYPE, "text/html".parse().unwrap());
        headers.insert(CONTENT_SECURITY_POLICY, HTML_CSP.parse().unwrap());

        page.render_html(ctx).await
    };

    (StatusCode::OK, headers, content)
}

const FAVICON_STR: &[u8] = include_bytes!("assets/favicon.ico");
async fn get_favicon() -> impl axum::response::IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "image/x-icon".parse().unwrap());
    (StatusCode::OK, headers, FAVICON_STR)
}

// async fn handle_cspreport(payload: String) {
//     println!("{}", payload);
// }

// 404 handler
async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, format!("no route {}", uri))
}

// GET /
async fn get_root(
    TypedHeader(user_agent): TypedHeader<UserAgent>,
) -> impl axum::response::IntoResponse {
    render(user_agent, Pages::Root).await
}

// GET /info
async fn get_info(
    TypedHeader(user_agent): TypedHeader<UserAgent>,
) -> impl axum::response::IntoResponse {
    render(user_agent, Pages::Info).await
}

// GET /upcoming
async fn get_upcoming(
    TypedHeader(user_agent): TypedHeader<UserAgent>,
) -> impl axum::response::IntoResponse {
    render(user_agent, Pages::Upcoming).await
}

// GET /lastseen
async fn get_lastseen(
    TypedHeader(user_agent): TypedHeader<UserAgent>,
) -> impl axum::response::IntoResponse {
    render(user_agent, Pages::LastSeen).await
}
