use axum::http::StatusCode;

#[tracing::instrument(name = "Health Check")]
pub async fn health() -> StatusCode {
    StatusCode::OK
}
