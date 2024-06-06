use crate::repository::auth::AuthRepository;
use crate::state::ApplicationState;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::IntoResponse;
use metrics::counter;
use sha3::{Digest, Sha3_256};
use std::sync::Arc;

pub async fn auth(
    State(state): State<Arc<ApplicationState>>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let labels = [("uri", format!("{}!", req.uri()))];
    let api_key = req
        .headers()
        .get("x-api-key")
        .map(|value| value.to_str().unwrap_or_default())
        .ok_or_else(|| {
            tracing::error!("Unauthorized call to API: No key header received");
            counter!("unauthenticated_calls_count", &labels);
            (StatusCode::UNAUTHORIZED, "Unautohorized".into())
        })?;
    let setting = state.auth_repository.create_auth().await?;
    let mut hasher = Sha3_256::new();
    hasher.update(api_key.as_bytes());
    let provided_api_key = hasher.finalize();

    if setting.encrypted_global_api_key != format!("{provided_api_key:x}") {
        tracing::error!("Unauthenticated call to API: Incorrect key supplied");
        counter!("unauthenticated_calls_count", &labels);
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".into()));
    }
    Ok(next.run(req).await)
}
