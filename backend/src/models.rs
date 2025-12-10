use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::types::Json;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "blink_type", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum BlinkType {
    Donation,
    Payment,
    Vote,
}

#[derive(Debug, FromRow, Serialize)]
pub struct Blink {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub title: String,
    pub icon_url: String,
    pub description: String,
    pub label: String,
    pub wallet_address: String,
    pub r#type: BlinkType,
    pub config: Json<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct CreateBlinkRequest {
    pub title: String,
    pub icon_url: String,
    pub description: String,
    pub label: String,
    pub wallet_address: String,
    pub r#type: BlinkType,
    pub config: serde_json::Value,
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
    pub links: Option<ActionLinks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ActionLinks {
    pub actions: Vec<LinkedAction>,
}

#[derive(Debug, Serialize)]
pub struct LinkedAction {
    pub label: String,
    pub href: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ActionParameter>>,
}

#[derive(Debug, Serialize)]
pub struct ActionParameter {
    pub name: String,
    pub label: Option<String>,
    pub required: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ActionPostRequest {
    pub account: String,
}

#[derive(Debug, Deserialize)]
pub struct ActionQueryParams {
    pub amount: Option<String>,
    pub selection: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ActionPostResponse {
    pub transaction: String,
    pub message: Option<String>,
}
