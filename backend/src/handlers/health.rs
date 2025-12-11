use axum::http::StatusCode;

#[tracing::instrument(name = "Health Check")]
pub async fn health() -> StatusCode {
    StatusCode::OK
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn health_check_works() {
        let response = health().await;
        assert_eq!(response, StatusCode::OK);
    }
}
