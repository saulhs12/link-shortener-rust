use crate::api::utils::{internal_error, timeout_duration};
use crate::repository::AuthPostgresRepository;
use axum::http::StatusCode;
pub struct Auth {
    #[allow(dead_code)]
    pub id: String,
    pub encrypted_global_api_key: String,
}

pub trait AuthRepository {
    async fn create_auth(&self) -> Result<Auth, (StatusCode, String)>;
}

impl AuthRepository for AuthPostgresRepository {
    async fn create_auth(&self) -> Result<Auth, (StatusCode, String)> {
        let fetch_setting_timeout = timeout_duration();
        let setting = tokio::time::timeout(
            fetch_setting_timeout,
            sqlx::query_as!(
                Auth,
                "select id, encrypted_global_api_key from settings where id = $1",
                "DEFAULT_SETTINGS"
            )
            .fetch_one(&self.pool),
        )
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

        Ok(setting)
    }
}
