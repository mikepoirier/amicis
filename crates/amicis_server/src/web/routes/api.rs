use std::collections::HashMap;
use axum::{response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;
use crate::web::routes::ResourceType::{CorrespondenceOverview, CorrespondenceTeaser, TransmissionTeaser};

use super::{Document, health_status, HealthStatus, Link, Resource};

pub fn routes() -> Router {
    Router::new()
        .route("/health", get(get_health))
        .route("/correspondences", get(get_correspondences))
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: HealthStatus,
}

async fn get_health() -> impl IntoResponse {
    let status = health_status().await;
    Json(HealthResponse { status })
}

async fn get_correspondences() -> impl IntoResponse {
    let mut correspondences = HashMap::new();
    correspondences.insert(
        "correspondences".to_string(),
        vec![
            Link{
                rel: "item".to_string(),
                href: "/correspondences/1".to_string(),
                target_title: "Keeping in touch".to_string(),
                target_type: None,
            },
            Link{
                rel: "item".to_string(),
                href: "/correspondences/2".to_string(),
                target_title: "Amicis development".to_string(),
                target_type: None,
            }],
    );
    let mut transmissions = HashMap::new();
    transmissions.insert(
        "transmissions".to_string(),
        vec![
            Link{
                rel: "item".to_string(),
                href: "/transmission/d7c80354-a8a9-4a02-ac3d-da79f9b64150".to_string(),
                target_title: "Trying this out".to_string(),
                target_type: None,
            },
    ],
    );
    Json(Document{
        data: Resource{
            kind: CorrespondenceOverview,
            uri: "/correspondences".to_string(),
            attributes: HashMap::from([
                ("title".to_string(), "Correspondences".to_string())
            ]),
            relationships: Some(correspondences),
        },
        included: Some(vec![
            Resource{
                kind: CorrespondenceTeaser,
                uri: "/correspondences/1".to_string(),
                attributes: HashMap::from([
                    ("title".to_string(), "Keeping in touch".to_string()),
                ]),
                relationships: Some(transmissions),
            },
            Resource{
                kind: CorrespondenceTeaser,
                uri: "/correspondences/2".to_string(),
                attributes: HashMap::from([
                    ("title".to_string(), "Amicis development".to_string()),
                ]),
                relationships: None,
            },
            Resource{
                kind: TransmissionTeaser,
                uri: "/transmission/d7c80354-a8a9-4a02-ac3d-da79f9b64150".to_string(),
                attributes: HashMap::from([
                    ("subject".to_string(), "Trying this out".to_string()),
                    ("body".to_string(), "Wazzup, bruh?".to_string()),
                ]),
                relationships: None,
            },
        ]),
    })
}
