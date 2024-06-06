use crate::api::utils::{internal_error, timeout_duration};
use crate::models::statistic::CountedLinkStastitic;
use crate::repository::StatisticsPostgresRepository;
use anyhow::Result;
use axum::http::StatusCode;

pub trait StatisticsRepository {
    async fn save_statistics(&self, link_id: &str, referer: &str, user_agent: &str);
    async fn fetch_statistics(
        &self,
        link_id: &String,
    ) -> Result<Vec<CountedLinkStastitic>, (StatusCode, String)>;
}

impl StatisticsRepository for StatisticsPostgresRepository {
    async fn save_statistics(&self, link_id: &str, referer: &str, user_agent: &str) {
        let insert_statistics_timeout = tokio::time::Duration::from_millis(300);
        let saved_statistics = tokio::time::timeout(
            insert_statistics_timeout,
            sqlx::query!(
                r#"
                insert into link_statistics(link_id,referer,user_agent)
                values($1,$2,$3)
                "#,
                &link_id,
                &referer,
                &user_agent
            )
            .execute(&self.pool),
        )
        .await;
        match saved_statistics {
            Ok(Err(err)) => tracing::error!("Saving a new link click failes"),
            Err(elapsed) => {
                tracing::error!("Saving new link click resulted in a timeout: {}", elapsed)
            }
            _ => tracing::debug!(
                "Persisted new link click for link with id {}, referer {} and user_agent {}",
                &link_id,
                referer,
                user_agent
            ),
        }
    }

    async fn fetch_statistics(
        &self,
        link_id: &String,
    ) -> Result<Vec<CountedLinkStastitic>, (StatusCode, String)> {
        let fetch_statistics_timeout = timeout_duration();
        let statistics = tokio::time::timeout(fetch_statistics_timeout,
        sqlx::query_as!(
            CountedLinkStastitic,
            r#"
             select count(*) as amount, referer,user_agent from link_statistics group by link_id,referer,user_agent having link_id = $1
            "#,link_id
        ).fetch_all(&self.pool))
            .await
            .map_err(internal_error)?
            .map_err(internal_error)?;

        tracing::debug!("Statistics for link with id {} requested", link_id);
        Ok(statistics)
    }
}
