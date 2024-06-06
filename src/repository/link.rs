use crate::api::utils::{internal_error};
use crate::models::link::Link;
use crate::repository::LinkPostgresRepository;
use crate::services::generate_id::{GenerateLink, GenerateLinkService};
use anyhow::Result;
use axum::http::StatusCode;
use axum::Json;
use sqlx::error::ErrorKind;
use sqlx::{query_as, Error};

pub trait LinkRepository {
    async fn fetch_link_requested(&self, link_id: String) -> Result<Link, (StatusCode, String)>;
    async fn insert_link(&self, url: &String) -> Result<Link, (StatusCode, String)>;
    async fn update_link(
        &self,
        link_id: &String,
        url: &String,
    ) -> Result<Link, (StatusCode, String)>;
}

impl LinkRepository for LinkPostgresRepository {
    async fn fetch_link_requested(&self, link_id: String) -> Result<Link, (StatusCode, String)> {
        let select_timeout = tokio::time::Duration::from_millis(300);

        let link = tokio::time::timeout(
            select_timeout,
            query_as!(
                Link,
                r#"
            SELECT id, target_url from links where id =$1
            "#,
                link_id
            )
            .fetch_optional(&self.pool),
        )
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?
        .ok_or_else(|| "Not found".to_string())
        .map_err(|err| (StatusCode::NOT_FOUND, err))?;

        Ok(link)
    }

    async fn insert_link(&self, url: &String) -> Result<Link, (StatusCode, String)> {
        let insert_link_timeout = tokio::time::Duration::from_millis(300);

        let link_id = GenerateLinkService::generate_id();
        let new_link = tokio::time::timeout(
            insert_link_timeout,
            query_as!(
                Link,
                r#"
                with inserted_link as (
                insert into links(id, target_url)
                values ($1, $2)
                returning id,target_url)
                select id,target_url from inserted_link
                "#,
                &link_id,
                &url
            )
            .fetch_one(&self.pool),
        )
        .await
        .map_err(internal_error)?;
        match new_link {
            Ok(link) => {
                tracing::debug!("Created new link with id {} targeting {}", link_id, url);

                Ok(link)
            }
            Err(err) => match err {
                Error::Database(db_err) if db_err.kind() == ErrorKind::UniqueViolation => {
                    Err(internal_error(db_err))
                }
                _ => Err(internal_error(err)),
            },
        }
    }

    async fn update_link(
        &self,
        link_id: &String,
        url: &String,
    ) -> Result<Link, (StatusCode, String)> {
        let update_link_timeout = tokio::time::Duration::from_millis(300);
        let link = tokio::time::timeout(
            update_link_timeout,
            sqlx::query_as!(
                Link,
                r#"
                with updated_link as (
                update links set target_url = $1 where id = $2
                returning id,target_url
                )
                select id, target_url
                from updated_link
                "#,
                &url,
                &link_id
            )
            .fetch_one(&self.pool),
        )
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

        tracing::debug!("Updated link with id {}, now targeting {}", link_id, url);

        Ok(link)
    }
}
