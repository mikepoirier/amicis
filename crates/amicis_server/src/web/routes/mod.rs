use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

use serde::Serialize;

pub mod api;
pub mod ui;

#[derive(Debug, Serialize)]
#[allow(unused)]
enum HealthStatus {
    Up,
    Down,
}

impl Display for HealthStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let status = match self {
            HealthStatus::Up => "Up",
            HealthStatus::Down => "Down",
        };
        write!(f, "{}", status)
    }
}

#[derive(Debug, Serialize)]
enum ResourceType {
    CorrespondenceOverview,
    CorrespondenceTeaser,
    TransmissionTeaser,
}

impl Display for ResourceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let status = match self {
            ResourceType::CorrespondenceOverview => "correspondence:overview",
            ResourceType::CorrespondenceTeaser => "correspondence:teaser",
            ResourceType::TransmissionTeaser => "transmission:teaser",
        };
        write!(f, "{}", status)
    }
}

async fn health_status() -> HealthStatus {
    HealthStatus::Up
}

#[derive(Debug, Serialize)]
struct Document {
    data: Resource,
    included: Option<Vec<Resource>>,
}

#[derive(Debug, Serialize)]
struct Resource {
    kind: ResourceType,
    uri: String,
    attributes: HashMap<String, String>,
    relationships: Option<HashMap<String, Vec<Link>>>,
}

#[derive(Debug, Serialize, Hash, Eq, PartialEq)]
struct Link {
    rel: String,
    href: String,
    target_title: String,
    target_type: Option<String>,
}
