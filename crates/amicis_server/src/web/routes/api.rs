use axum::{response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;
use std::collections::HashMap;
use uuid::Uuid;

use super::{health_status, Document, HealthStatus, Link, Resource, ResourceType};

pub fn routes() -> Router {
    Router::new()
        .route("/health", get(get_health))
        .route("/correspondences", get(get_correspondences))
        .route("/home-example", get(get_home_screen_example))
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
            Link {
                rel: "item".to_string(),
                href: "/correspondences/1".to_string(),
                target_title: "Keeping in touch".to_string(),
                target_type: None,
            },
            Link {
                rel: "item".to_string(),
                href: "/correspondences/2".to_string(),
                target_title: "Amicis development".to_string(),
                target_type: None,
            },
        ],
    );
    let mut transmissions = HashMap::new();
    transmissions.insert(
        "transmissions".to_string(),
        vec![Link {
            rel: "item".to_string(),
            href: "/transmission/d7c80354-a8a9-4a02-ac3d-da79f9b64150".to_string(),
            target_title: "Trying this out".to_string(),
            target_type: None,
        }],
    );
    Json(Document {
        data: Resource {
            kind: ResourceType::CorrespondenceOverview,
            uri: "/correspondences".to_string(),
            attributes: HashMap::from([("title".to_string(), "Correspondences".to_string())]),
            relationships: Some(correspondences),
        },
        included: Some(vec![
            Resource {
                kind: ResourceType::CorrespondenceTeaser,
                uri: "/correspondences/1".to_string(),
                attributes: HashMap::from([("title".to_string(), "Keeping in touch".to_string())]),
                relationships: Some(transmissions),
            },
            Resource {
                kind: ResourceType::CorrespondenceTeaser,
                uri: "/correspondences/2".to_string(),
                attributes: HashMap::from([(
                    "title".to_string(),
                    "Amicis development".to_string(),
                )]),
                relationships: None,
            },
            Resource {
                kind: ResourceType::TransmissionTeaser,
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

static CORRESPONDENCE_TEASER_TYPE: &str = "correspondence:teaser";

#[derive(Debug, Clone)]
struct CorrespondenceTeaser {
    id: String,
    title: String,
    transmissions: Vec<TransmissionTeaser>,
}

impl CorrespondenceTeaser {
    fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        transmissions: Vec<TransmissionTeaser>,
    ) -> Self {
        let id = id.into();
        let title = title.into();
        Self {
            id,
            title,
            transmissions,
        }
    }
}

impl From<CorrespondenceTeaser> for json_api::v2::Resource {
    fn from(value: CorrespondenceTeaser) -> Self {
        let mut builder = json_api::v2::Resource::builder()
            .kind(CORRESPONDENCE_TEASER_TYPE)
            .uri(format!("/api/correspondences/{}", value.id))
            .attribute("title", value.title)
            .attribute("transmission_count", value.transmissions.len());

        if !value.transmissions.is_empty() {
            builder = builder.relationship(
                "transmissions",
                json_api::v2::Relationship::from_vec(value.transmissions),
            )
        }
        builder.build()
    }
}

impl From<CorrespondenceTeaser> for json_api::v2::RelationshipObject {
    fn from(value: CorrespondenceTeaser) -> Self {
        json_api::v2::RelationshipObject::builder()
            .rel("item")
            .href(format!("/api/correspondences/{}", value.id))
            .extension("title", value.title)
            .build()
    }
}

static CORRESPONDENCE_OVERVIEW_TYPE: &str = "correspondence:overview";

#[derive(Debug, Default)]
struct CorrespondenceOverview(Vec<CorrespondenceTeaser>);

impl From<CorrespondenceOverview> for json_api::v2::Resource {
    fn from(value: CorrespondenceOverview) -> Self {
        json_api::v2::Resource::builder()
            .kind(CORRESPONDENCE_OVERVIEW_TYPE)
            .uri("/api/correspondences")
            .attribute("title", "Correspondences")
            .relationship(
                "correspondences",
                json_api::v2::Relationship::from_vec(value.0),
            )
            .build()
    }
}

#[derive(Debug, Clone)]
struct TransmissionTeaser {
    id: Uuid,
    body: String,
    subject: String,
}

impl TransmissionTeaser {
    fn new(body: impl Into<String>, subject: impl AsRef<str>) -> Self {
        let id = Uuid::new_v4();
        let body = body.into();
        let full_subject = subject.as_ref();
        let subject = format!("{}...", &full_subject[..65]);
        Self { id, body, subject }
    }
}

impl From<TransmissionTeaser> for json_api::v2::Resource {
    fn from(value: TransmissionTeaser) -> Self {
        json_api::v2::Resource::builder()
            .kind("transmission:teaser")
            .uri(format!("/api/transmissions/{}", value.id))
            .attribute("body", value.body)
            .attribute("subject", value.subject)
            .build()
    }
}

impl From<TransmissionTeaser> for json_api::v2::RelationshipObject {
    fn from(value: TransmissionTeaser) -> Self {
        json_api::v2::RelationshipObject::builder()
            .rel("item")
            .href(format!("/api/transmissions/{}", value.id))
            .extension("title", "Trying this out")
            .build()
    }
}

async fn get_home_screen_example() -> impl IntoResponse {
    let transmission_teaser = TransmissionTeaser::new(
        "Wazzup, bruh?",
        "Trying this out. Here is some snipped content for the preview text. This will be cut out.",
    );
    let correspondence_teasers = vec![
        CorrespondenceTeaser::new("1", "Keeping in touch", vec![transmission_teaser.clone()]),
        CorrespondenceTeaser::new("2", "Amicis development", vec![]),
    ];
    let overview = CorrespondenceOverview(correspondence_teasers.clone());
    let document = json_api::v2::Document::builder()
        .data(overview)
        .includes_all(correspondence_teasers)
        .includes(transmission_teaser)
        .build();
    Json(document)
}
