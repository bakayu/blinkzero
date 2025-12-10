use axum::{Json, extract::State, http::StatusCode};
use sqlx::PgPool;

use crate::models::{Blink, CreateBlinkRequest, CreateBlinkResponse};

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
        std::env::var("BACKEND_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

    let blink: Blink = sqlx::query_as(
        r#"
        INSERT INTO blinks (title, icon_url, description, label, wallet_address, amount_sol)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#,
    )
    .bind(&payload.title)
    .bind(&payload.icon_url)
    .bind(&payload.description)
    .bind(&payload.label)
    .bind(&payload.wallet_address)
    .bind(payload.amount_sol)
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(CreateBlinkResponse {
        id: blink.id,
        action_url: format!("{}/api/actions/{}", backend_url, blink.id),
    }))
}
