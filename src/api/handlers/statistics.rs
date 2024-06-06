use crate::models::statistic::CountedLinkStastitic;
use crate::repository::statistics::StatisticsRepository;
use crate::state::ApplicationState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use std::sync::Arc;

pub async fn get_link_statistics(
    State(state): State<Arc<ApplicationState>>,
    Path(link_id): Path<String>,
) -> Result<Json<Vec<CountedLinkStastitic>>, (StatusCode, String)> {
    let statistics = state
        .statistics_repository
        .fetch_statistics(&link_id)
        .await?;
    Ok(Json(statistics))
}
