use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use solana_client::nonblocking::rpc_client::RpcClient;
use std::time::Duration;

use solana_sdk::{
    message::Message, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, system_instruction,
    transaction::Transaction,
};
use sqlx::PgPool;
use std::str::FromStr;
use uuid::Uuid;

use crate::models::{
    ActionLink, ActionLinks, ActionMetadata, ActionPostRequest, ActionPostResponse, Blink,
};

#[tracing::instrument(
    name = "Fetching action metadata",
    skip(pool),
    fields(blink_id = %id)
)]
pub async fn get_action_metadata(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<ActionMetadata>, (StatusCode, String)> {
    let backend_url =
        std::env::var("BACKEND_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

    let blink = fetch_blink(&pool, id).await?;

    Ok(Json(ActionMetadata {
        icon: blink.icon_url,
        label: blink.label.clone(),
        title: blink.title,
        description: blink.description,
        links: ActionLinks {
            actions: vec![ActionLink {
                label: blink.label,
                href: format!("{}/blinks/{}", backend_url, id),
            }],
        },
    }))
}

#[tracing::instrument(
    name = "Building action transaction",
    skip(pool),
    fields(blink_id = %id, account = %payload.account)
)]
pub async fn post_action_transaction(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<ActionPostRequest>,
) -> Result<Json<ActionPostResponse>, (StatusCode, String)> {
    let blink = fetch_blink(&pool, id).await?;
    let user_pubkey = parse_pubkey(&payload.account, "user wallet")?;
    let destination_pubkey = parse_pubkey(&blink.wallet_address, "destination wallet")?;

    let transaction =
        build_transfer_transaction(&user_pubkey, &destination_pubkey, blink.amount_sol).await?;

    let serialized = bincode::serialize(&transaction).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Serialization error: {}", e),
        )
    })?;

    Ok(Json(ActionPostResponse {
        transaction: BASE64.encode(&serialized),
        message: Some(format!("Send {} SOL to {}", blink.amount_sol, blink.title)),
    }))
}

async fn fetch_blink(pool: &PgPool, id: Uuid) -> Result<Blink, (StatusCode, String)> {
    sqlx::query_as("SELECT * FROM blinks WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Blink not found".to_string()))
}

fn parse_pubkey(address: &str, name: &str) -> Result<Pubkey, (StatusCode, String)> {
    Pubkey::from_str(address)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid {}: {}", name, e)))
}

async fn build_transfer_transaction(
    from: &Pubkey,
    to: &Pubkey,
    amount_sol: f64,
) -> Result<Transaction, (StatusCode, String)> {
    let rpc_url = std::env::var("RPC_URL").map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "RPC_URL not set".to_string(),
        )
    })?;

    tracing::info!("Connecting to RPC: {}", rpc_url);
    let client = RpcClient::new_with_timeout(rpc_url, Duration::from_secs(30));

    tracing::info!("Fetching latest blockhash...");
    let recent_blockhash = client.get_latest_blockhash().await.map_err(|e| {
        tracing::error!("RPC error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("RPC error: {}", e),
        )
    })?;
    tracing::info!("Got blockhash: {}", recent_blockhash);

    let lamports = (amount_sol * LAMPORTS_PER_SOL as f64) as u64;
    let instruction = system_instruction::transfer(from, to, lamports);

    let message = Message::new_with_blockhash(&[instruction], Some(from), &recent_blockhash);

    let transaction = Transaction::new_unsigned(message);

    Ok(transaction)
}
