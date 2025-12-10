use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    compute_budget::ComputeBudgetInstruction,
    instruction::{AccountMeta, Instruction},
    message::Message,
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    system_instruction,
    transaction::Transaction,
};
use sqlx::PgPool;
use std::str::FromStr;
use uuid::Uuid;

use crate::models::{
    ActionLinks, ActionMetadata, ActionParameter, ActionPostRequest, ActionPostResponse,
    ActionQueryParams, Blink, BlinkType, LinkedAction,
};

const MEMO_PROGRAM_ID: &str = "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr";

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
        std::env::var("BACKEND_URL").unwrap_or_else(|_| "http://localhost:8000".to_string());

    let blink = fetch_blink(&pool, id).await?;

    let actions = match blink.r#type {
        BlinkType::Donation => vec![LinkedAction {
            label: blink.label.clone(),
            href: format!(
                "{}/api/actions/{}?amount={}",
                backend_url,
                id,
                blink
                    .config
                    .get("amount")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.1)
            ),
            parameters: None,
        }],
        BlinkType::Payment => vec![LinkedAction {
            label: "Send SOL".to_string(),
            href: format!("{}/api/actions/{}?amount={{amount}}", backend_url, id),
            parameters: Some(vec![ActionParameter {
                name: "amount".to_string(),
                label: Some("Enter SOL amount".to_string()),
                required: Some(true),
            }]),
        }],
        BlinkType::Vote => {
            let options = blink
                .config
                .get("options")
                .and_then(|v| v.as_array())
                .ok_or((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Invalid vote config".to_string(),
                ))?;

            options
                .iter()
                .map(|opt| {
                    let val = opt.as_str().unwrap_or("Unknown");
                    LinkedAction {
                        label: format!("Vote {}", val),
                        href: format!("{}/api/actions/{}?selection={}", backend_url, id, val),
                        parameters: None,
                    }
                })
                .collect()
        }
    };

    Ok(Json(ActionMetadata {
        icon: blink.icon_url,
        label: blink.label.clone(),
        title: blink.title,
        description: blink.description,
        links: Some(ActionLinks { actions }),
        disabled: None,
    }))
}

#[tracing::instrument(
    name = "Building action transaction",
    skip(pool, params, payload),
    fields(blink_id = %id, account = %payload.account)
)]
pub async fn post_action_transaction(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Query(params): Query<ActionQueryParams>,
    Json(payload): Json<ActionPostRequest>,
) -> Result<Json<ActionPostResponse>, (StatusCode, String)> {
    let blink = fetch_blink(&pool, id).await?;
    let user_pubkey = parse_pubkey(&payload.account, "user wallet")?;

    let client = get_rpc_client()?;
    let recent_blockhash = client.get_latest_blockhash().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("RPC Error: {}", e),
        )
    })?;

    let (transaction, message) = match blink.r#type {
        BlinkType::Donation | BlinkType::Payment => {
            let destination_pubkey = parse_pubkey(&blink.wallet_address, "destination wallet")?;

            let amount: f64 = params
                .amount
                .as_ref()
                .and_then(|a| a.parse().ok())
                .or_else(|| blink.config.get("amount").and_then(|v| v.as_f64()))
                .ok_or((
                    StatusCode::BAD_REQUEST,
                    "Missing or invalid amount".to_string(),
                ))?;

            let tx = build_transfer_transaction(
                &user_pubkey,
                &destination_pubkey,
                amount,
                recent_blockhash,
            )?;
            let msg = format!("Send {} SOL to {}", amount, blink.title);
            (tx, msg)
        }
        BlinkType::Vote => {
            let selection = params
                .selection
                .as_ref()
                .ok_or((StatusCode::BAD_REQUEST, "Missing selection".to_string()))?;

            let tx = build_memo_transaction(&user_pubkey, id, selection, recent_blockhash)?;
            let msg = format!("Vote for: {}", selection);
            (tx, msg)
        }
    };

    let serialized = bincode::serialize(&transaction).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Serialization error: {}", e),
        )
    })?;

    Ok(Json(ActionPostResponse {
        transaction: BASE64.encode(&serialized),
        message: Some(message),
    }))
}

fn get_rpc_client() -> Result<RpcClient, (StatusCode, String)> {
    let rpc_url = std::env::var("RPC_URL").map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "RPC_URL not set".to_string(),
        )
    })?;
    Ok(RpcClient::new_with_commitment(
        rpc_url,
        CommitmentConfig::confirmed(),
    ))
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

fn build_transfer_transaction(
    from: &Pubkey,
    to: &Pubkey,
    amount_sol: f64,
    recent_blockhash: solana_sdk::hash::Hash,
) -> Result<Transaction, (StatusCode, String)> {
    let lamports = (amount_sol * LAMPORTS_PER_SOL as f64) as u64;
    let transfer_ix = system_instruction::transfer(from, to, lamports);
    let priority_fee_ix = ComputeBudgetInstruction::set_compute_unit_price(50_000);

    let message = Message::new_with_blockhash(
        &[priority_fee_ix, transfer_ix],
        Some(from),
        &recent_blockhash,
    );
    Ok(Transaction::new_unsigned(message))
}

fn build_memo_transaction(
    from: &Pubkey,
    blink_id: Uuid,
    selection: &str,
    blockhash: solana_sdk::hash::Hash,
) -> Result<Transaction, (StatusCode, String)> {
    let memo_program_id = Pubkey::from_str(MEMO_PROGRAM_ID).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Invalid memo program ID: {}", e),
        )
    })?;

    let memo_data = format!("vote:{}:{}", blink_id, selection);

    let memo_ix = Instruction {
        program_id: memo_program_id,
        accounts: vec![AccountMeta::new_readonly(*from, true)],
        data: memo_data.into_bytes(),
    };

    let priority_fee_ix = ComputeBudgetInstruction::set_compute_unit_price(50_000);

    let message = Message::new_with_blockhash(&[priority_fee_ix, memo_ix], Some(from), &blockhash);
    Ok(Transaction::new_unsigned(message))
}
