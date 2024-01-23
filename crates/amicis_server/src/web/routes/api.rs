use axum::{response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;

use super::{health_status, HealthStatus};

pub fn routes() -> Router {
    Router::new().route("/health", get(get_health))
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: HealthStatus,
}

async fn get_health() -> impl IntoResponse {
    let status = health_status().await;
    Json(HealthResponse { status })
}
