use axum::{
    BoxError,
    error_handling::HandleErrorLayer,
    http::{ Method, StatusCode },
    Router,
};
use axum_sessions::{SessionLayer, SameSite};
use dotenvy::dotenv;
use http::{ HeaderValue, header::{ HeaderName, ACCEPT, ACCEPT_ENCODING, AUTHORIZATION, CONTENT_TYPE } };
use std::{ env, time::Duration };
use tower::{ ServiceBuilder, timeout::TimeoutLayer };
use tower_governor::{ errors::display_error, governor::GovernorConfigBuilder, GovernorLayer };
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer, 
    trace::{ DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer },
    request_id::{ SetRequestIdLayer, PropagateRequestIdLayer },
};
use tracing::Level;

use crate::RequestId;
use crate::sessionstore::PostgresSessionStore;

pub fn with_middleware_stack(service: Router) -> Router {
    // security
    let cors_layer = CorsLayer::new()
        .allow_methods([
            Method::GET, 
            Method::POST,
            Method::PUT
        ])
        .allow_headers([
            ACCEPT,
            ACCEPT_ENCODING,
            AUTHORIZATION,
            CONTENT_TYPE,
        ])
        .allow_origin([
            "http://::1:3000".parse::<HeaderValue>().unwrap(),
            "http://0.0.0.0:3000".parse::<HeaderValue>().unwrap(),
            "http://127.0.0.1:3000".parse::<HeaderValue>().unwrap(),
            "http://localhost:3000".parse::<HeaderValue>().unwrap(),
        ])
        .allow_credentials(true);

    // 2 req per sec
    let governor_conf = Box::new(
        GovernorConfigBuilder::default()
            .per_second(2)
            .burst_size(5)
            .finish()
            .unwrap(),
    );

    let governor_layer = ServiceBuilder::new()
        // this middleware goes above `GovernorLayer` because it will receive
        // errors returned by `GovernorLayer`
        .layer(HandleErrorLayer::new(|e: BoxError| async move {
            display_error(e)
        }))
        .layer(GovernorLayer {
            // We can leak this because it is created once and then
            config: Box::leak(governor_conf),
        });

    let timeout_layer = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|e: BoxError| async move {
            if e.is::<tower::timeout::error::Elapsed>() {
                (
                    StatusCode::REQUEST_TIMEOUT,
                    e.to_string(),
                )
            } else {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    e.to_string(),
                )
            }
        }))
        .layer(TimeoutLayer::new(Duration::from_secs(10)));

    // logging
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().include_headers(true))
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO));

    let x_request_id = HeaderName::from_static("x-request-id");

    let request_id_layer = ServiceBuilder::new()
        .layer(SetRequestIdLayer::new(
            x_request_id.clone(),
            RequestId::default(),
        ))
        .layer(PropagateRequestIdLayer::new(x_request_id));

    // session
    dotenv().ok();
    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let secret = env::var("SESSION_SECRET")
        .expect("SESSION_SECRET must be set");

    let store = PostgresSessionStore::new(db_url);

    let session_layer = SessionLayer::new(store, secret.as_bytes())
        .with_cookie_name("sid")
        .with_cookie_domain("127.0.0.1")
        .with_same_site_policy(SameSite::Strict)
        .with_session_ttl(Some(std::time::Duration::from_secs(60 * 60 * 8)))
        .with_secure(true);
    
    // data
    let compression_layer = CompressionLayer::new().gzip(true);

    service
        .layer(compression_layer)
        .layer(session_layer)
        .layer(trace_layer)
        .layer(request_id_layer)
        .layer(timeout_layer)
        .layer(governor_layer)
        .layer(cors_layer)
}