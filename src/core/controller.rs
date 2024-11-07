use std::{sync::Arc, time::Duration};

use axum::{
    http::{header, HeaderName, HeaderValue},
    Extension, Router,
};
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    set_header::SetResponseHeaderLayer,
    timeout::TimeoutLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};

use crate::{index, middleware::auth_token, user};

use super::{config::Config, AppState};

/// 路由规则
/// ```
/// use axum::{routing::get, Router};
/// async fn handler() {}
/// let app = Router::new()
///    .route("/", get(handler))
///    .layer(role_auth)
///    .layer(bearer_auth)
///    .route("/whites",get(handler))
///    .layer(public_layer)
/// ```
pub fn init() -> Router {
    let config = Config::init();
    tracing::info!(?config);
    let app_state: Arc<AppState> = Arc::new(AppState { conn: 1, config });

    let public_layer = tower::ServiceBuilder::new()
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(tracing::Level::INFO))
                .on_request(DefaultOnRequest::new().level(tracing::Level::INFO))
                .on_response(DefaultOnResponse::new().level(tracing::Level::INFO)),
        )
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive())
        .layer(SetResponseHeaderLayer::appending(
            HeaderName::from_static("x-service-version"),
            HeaderValue::from_static(env!("CARGO_PKG_VERSION")),
        ))
        .layer(SetResponseHeaderLayer::appending(
            HeaderName::from_static("x-response-service"),
            HeaderValue::from_static(env!("CARGO_PKG_NAME")),
        ))
        .layer(SetResponseHeaderLayer::appending(
            header::SERVER,
            HeaderValue::from_static("Axum"),
        ))
        .layer(TimeoutLayer::new(Duration::from_secs(60)));
    Router::new()
        .merge(user::controller())
        .route_layer(axum::middleware::map_request_with_state(
            app_state.clone(),
            auth_token,
        ))
        .merge(index::controller())
        .layer(Extension(app_state))
        .route("/whites", axum::routing::get(|| async { "whites" }))
        .layer(public_layer)
}
