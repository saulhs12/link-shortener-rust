use crate::models::link::{Link, LinkTarget};
use crate::repository::link::LinkRepository;
use crate::repository::statistics::StatisticsRepository;
use crate::services::generate_id::{ GenerateLinkService};
use crate::state::ApplicationState;
use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, Response, StatusCode};
use axum::Json;
use axum_prometheus::metrics::counter;

use std::sync::Arc;
use url::Url;

const DEFAULT_CACHE_CONTROL_HEADER_VALUE: &str =
    "public, max-age=300 s-maxage=300,stale-while-revalidate=300,stale-if-error=300";

pub async fn redirect(
    State(state): State<Arc<ApplicationState>>,
    Path(requested_link): Path<String>,
    headers: HeaderMap,
) -> Result<Response<Body>, (StatusCode, String)> {
    let link = state
        .link_repository
        .fetch_link_requested(requested_link)
        .await?;
    let referer_header = headers
        .get("referer")
        .map(|value| value.to_str().unwrap_or_default().to_string());
    let user_agent_header = headers
        .get("user-agent")
        .map(|value| value.to_str().unwrap_or_default().to_string());
    state
        .statistics_repository
        .save_statistics(
            &link.id,
            &referer_header.unwrap_or_default(),
            &user_agent_header.unwrap_or_default(),
        )
        .await;
    Ok(Response::builder()
        .status(StatusCode::TEMPORARY_REDIRECT)
        .header("Location", link.target_url)
        .header("Cache-Control", DEFAULT_CACHE_CONTROL_HEADER_VALUE)
        .body(Body::empty())
        .expect("This response should always be constructable"))
}

pub async fn create_link(
    State(state): State<Arc<ApplicationState>>,
    Json(new_link): Json<LinkTarget>,
) -> Result<Json<Link>, (StatusCode, String)> {
    let url = Url::parse(&new_link.target_url)
        .map_err(|_| (StatusCode::CONFLICT, "url malformed".into()))?
        .to_string();

    if (1..=3).next().is_some() {
        let new_link = state.link_repository.insert_link(&url).await;
        return match new_link {
            Ok(link) => {
                tracing::debug!("Created new link with id {} targeting {}", &link.id, url);

                Ok(Json(link))
            }
            Err(err) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".into(),
            )),
        };
    }

    tracing::error!(
        "Could not persist new short link. Exhausted all retries of generating a unique id"
    );
    counter!("saving_link_impossible_no_unique_id");

    Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        "Internal server error".into(),
    ))
}

pub async fn update_link(
    State(state): State<Arc<ApplicationState>>,
    Path(link_id): Path<String>,
    Json(update_link): Json<LinkTarget>,
) -> Result<Json<Link>, (StatusCode, String)> {
    let url = Url::parse(&update_link.target_url)
        .map_err(|_| (StatusCode::CONFLICT, "url malformed".into()))?
        .to_string();
    let updated_link = state.link_repository.update_link(&link_id, &url).await?;
    Ok(Json(updated_link))
}
