use crate::models::{Blink, BlinkType, CreateBlinkRequest, CreateBlinkResponse};
use axum::{Json, extract::State, http::StatusCode};
use sqlx::PgPool;

#[tracing::instrument(
    name = "Creating a new blink",
    skip(pool),
    fields(
        blink_title = %payload.title,
        wallet = %payload.wallet_address
    )
)]
pub async fn create_blink(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateBlinkRequest>,
) -> Result<Json<CreateBlinkResponse>, (StatusCode, String)> {
    let backend_url =
        std::env::var("BACKEND_URL").unwrap_or_else(|_| "http://localhost:8000".to_string());

    let blink = sqlx::query_as!(
        Blink,
        r#"
        INSERT INTO blinks (title, icon_url, description, label, wallet_address, type, config)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING 
            id, 
            created_at as "created_at!",
            title, 
            icon_url, 
            description, 
            label, 
            wallet_address, 
            type as "type: BlinkType", 
            config
        "#,
        payload.title,
        payload.icon_url,
        payload.description,
        payload.label,
        payload.wallet_address,
        payload.r#type as BlinkType,
        payload.config
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(CreateBlinkResponse {
        id: blink.id,
        action_url: format!("{}/api/actions/{}", backend_url, blink.id),
    }))
}
