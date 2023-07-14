pub mod jwt;
use std::net::SocketAddr;

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, COOKIE},
    HeaderValue, Method,
};
use axum::{extract::DefaultBodyLimit, routing::get, Router, Server};
use tower_http::cors::{self, CorsLayer};

pub fn cors_layer() -> CorsLayer {
    // CorsLayer::new()
    // .allow_origin(cors::AllowOrigin::any())
    // .allow_methods(cors::AllowMethods::any())
    // .allow_headers(cors::AllowHeaders::any())
    // .allow_credentials(cors::AllowCredentials::yes())

    // see https://docs.rs/tower-http/latest/tower_http/cors/index.html
    CorsLayer::new()
        // .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_origin(cors::AllowOrigin::predicate(|_origin: &HeaderValue, _request_head: &axum::http::request::Parts| true))
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE, COOKIE])
    // CorsLayer::new()
}

pub async fn listen(addr: SocketAddr) -> anyhow::Result<()> {
    let state = crate::app::AppState::init().await?;
    let app = Router::new()
        .route(
            "/",
            get(|claims: Option<jwt::Claims>| async move {
                let mut text = "Welcome".to_string();
                if let Some(claims) = claims {
                    text.push_str(&format!(", {:?}", claims.sub));
                }
                text
            }),
        )
        .nest("/", crate::app::router::compose())
        .with_state(state)
        .layer(DefaultBodyLimit::max(2 * 1024 * 1024))
        .layer(cors_layer());

    log::debug!("GraphiQL IDE: {}/graphql", &addr);
    Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}
