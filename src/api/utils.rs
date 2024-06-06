use crate::models::link::LinkTarget;
use axum::http::StatusCode;
use metrics::counter;
use url::Url;

pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    tracing::error!("{}", err);
    let labels = [("error", format!("{}!", err))];
    counter!("request_error", &labels);
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

pub fn url_parser(url: &LinkTarget) -> anyhow::Result<String, (StatusCode, String)> {
    let url_parsed = Url::parse(&url.target_url)
        .map_err(|_| (StatusCode::CONFLICT, "url malformed".into()))?
        .to_string();
    Ok(url_parsed)
}

pub fn timeout_duration() -> core::time::Duration {
    tokio::time::Duration::from_millis(300)
}
