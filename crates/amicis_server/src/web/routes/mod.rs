use std::fmt::{Display, Formatter, Result};

use serde::Serialize;

pub mod api;
pub mod ui;

#[derive(Debug, Serialize)]
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

async fn health_status() -> HealthStatus {
    HealthStatus::Up
}
