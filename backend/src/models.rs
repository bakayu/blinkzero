use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize)]
pub struct Blink {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub title: String,
    pub icon_url: String,
    pub description: String,
    pub label: String,
    pub wallet_address: String,
    pub amount_sol: f64,
}

#[derive(Debug, Deserialize)]
pub struct CreateBlinkRequest {
    pub title: String,
    pub icon_url: String,
    pub description: String,
    pub label: String,
    pub wallet_address: String,
    #[serde(default = "default_amount")]
    pub amount_sol: f64,
}

fn default_amount() -> f64 {
    0.1
}

#[derive(Debug, Serialize)]
pub struct CreateBlinkResponse {
    pub id: Uuid,
    pub action_url: String,
}

#[derive(Debug, Serialize)]
pub struct ActionMetadata {
    pub icon: String,
    pub label: String,
    pub title: String,
    pub description: String,
    pub links: ActionLinks,
}

#[derive(Debug, Serialize)]
pub struct ActionLinks {
    pub actions: Vec<ActionLink>,
}

#[derive(Debug, Serialize)]
pub struct ActionLink {
    pub label: String,
    pub href: String,
}
