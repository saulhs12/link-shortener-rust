use crate::api::handlers::auth::auth;
use crate::api::handlers::redirect::{create_link, redirect, update_link};
use crate::api::handlers::statistics::get_link_statistics;
use crate::state::ApplicationState;
use axum::extract::State;
use axum::http::{ Method, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, patch, post};
use axum::{middleware,  Router};
use axum_prometheus::PrometheusMetricLayer;
use std::sync::Arc;
use tower_http::cors::{CorsLayer, AllowOrigin};

use tower_http::trace::TraceLayer;


pub fn router(state: Arc<ApplicationState>) -> Router {
    let (prometheus_layer, metrics_handle) = PrometheusMetricLayer::pair();
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::list(vec![
            "https://link-shortener-saul-rust.vercel.app".parse().unwrap(),  // Permitir el origen específico
        ]))
        .allow_methods(vec![Method::GET, Method::POST, Method::PATCH])
        .allow_headers(vec![
            "Content-Type".parse().unwrap(),  // Permitir el header Content-Type
        ])
        .allow_credentials(true);
    Router::new()
        
        .route(
            "/:id",
           
                patch(update_link)
                .route_layer(middleware::from_fn_with_state(state.clone(), auth))
                    
        )
        .route("/:id/statistics", get(get_link_statistics))
        .route_layer(middleware::from_fn_with_state(state.clone(), auth))

        .route("/metrics", get(|| async move { metrics_handle.render() }))
        .route("/:id",get(redirect))
        .route("/health", get(health_handler))
        .route("/create", post(create_link))
        .layer(TraceLayer::new_for_http())
        .layer(prometheus_layer)
        .layer(cors)
        .with_state(state.clone())
}

pub async fn health_handler(State(state): State<Arc<ApplicationState>>) -> impl IntoResponse {
    (
        StatusCode::OK,
        format!(
            "SERVER IS OK\
        host: {}:{}\
        database: {}",
            state.settings.load().application.host,
            state.settings.load().application.port,
            state.settings.load().database.url_db
        ),
    )
}

