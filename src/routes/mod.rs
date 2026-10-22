pub mod api;
pub mod auth;
pub mod ui;

use auth::AppState;
use axum::{
    routing::{delete, get, patch, post, put},
    Router,
};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

pub fn create_router(state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        // Web UI and Health Checks
        .route("/", get(ui::index_handler))
        .route("/healthz", get(ui::health_handler))
        // Authentication routes
        .route("/api/auth/login", post(auth::login_handler))
        .route("/api/auth/logout", post(auth::logout_handler))
        .route("/api/auth/me", get(auth::me_handler))
        // Inventory API routes
        .route("/api/servidores", get(api::list_servidores_handler))
        .route("/api/servidores", post(api::create_servidor_handler))
        .route("/api/servidores/{id}", get(api::get_servidor_handler))
        .route("/api/servidores/{id}", put(api::update_servidor_handler))
        .route("/api/servidores/{id}", delete(api::delete_servidor_handler))
        .route("/api/servidores/{id}/credentials", patch(api::update_credentials_handler))
        .route("/api/audit-logs", get(api::list_audit_logs_handler))
        .route("/api/stats", get(api::get_stats_handler))
        .route("/api/export/csv", get(api::export_csv_handler))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
