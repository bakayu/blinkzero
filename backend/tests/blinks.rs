mod helpers;

use helpers::spawn_app;
use serde_json::json;

#[tokio::test]
async fn create_blink_returns_200_for_valid_data() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = json!({
        "title": "Save the Rainforest",
        "icon_url": "https://example.com/image.png",
        "description": "Donate to help save trees",
        "label": "Donate",
        "wallet_address": "AbC2...WalletAddress",
        "type": "donation",
        "config": {
            "amount": 0.5
        }
    });

    // Act
    let response = client
        .post(format!("{}/api/blinks", &app.address))
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT title, label FROM blinks")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved blink.");

    assert_eq!(saved.title, "Save the Rainforest");
    assert_eq!(saved.label, "Donate");
}

#[tokio::test]
async fn create_blink_returns_422_for_missing_data() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = json!({
        "title": "Missing fields",
    });

    // Act
    let response = client
        .post(format!("{}/api/blinks", &app.address))
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(422, response.status().as_u16());
}
