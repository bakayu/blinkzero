use reqwest::Client;
use serde_json::json;

mod helpers;
use helpers::spawn_app;

#[tokio::test]
async fn create_blink_returns_200_for_valid_data() {
    let app = spawn_app().await;
    let client = Client::new();

    let body = json!({
        "title": "Test Blink",
        "icon_url": "https://example.com/icon.png",
        "description": "A test blink",
        "label": "Donate",
        "wallet_address": "11111111111111111111111111111111",
        "type": "donation",
        "config": { "amount": 0.1 }
    });

    let response = client
        .post(format!("{}/api/blinks", &app.address))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    if response.status() != 200 {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        panic!("Expected 200, got {}. Body: {}", status, body);
    }

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn create_blink_returns_422_for_missing_data() {
    let app = spawn_app().await;
    let client = Client::new();

    let test_cases = vec![
        (json!({}), "empty body"),
        (json!({"title": "Test"}), "missing fields"),
    ];

    for (body, description) in test_cases {
        let response = client
            .post(format!("{}/api/blinks", &app.address))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            422,
            response.status().as_u16(),
            "Expected 422 for case: {}",
            description
        );
    }
}
