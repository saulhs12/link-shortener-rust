use crate::api::v1::router;
use axum_prometheus::PrometheusMetricLayer;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::error::Error;
use std::sync::Arc;
use tracing::level_filters::LevelFilter;
use tracing::Level;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::settings;

use crate::settings::Settings;
use crate::state::ApplicationState;

#[tokio::main]
pub async fn start_server(settings: &Settings) -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "link_shortener=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = &settings.database.url_db.clone();
    
    tracing::debug!("Start DB conection");
    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(20)
        .connect(db_url)
        .await?;
    // sqlx::migrate!("./migrations").run(&pool).await?;
    let state = Arc::new(ApplicationState::new(settings, pool)?);
    let addr = format!(
        "{}:{}",
        &settings.application.host, &settings.application.port
    );
    let listener = tokio::net::TcpListener::bind(addr).await?;
    let router = router(state);

    tracing::debug!("Iniciar server");
    tracing::debug!("db url: {}", &settings.database.url_db);
    tracing::debug!(
        "host: {}:{}",
        &settings.application.host,
        &settings.application.port
    );
    axum::serve(listener, router.into_make_service()).await?;

    Ok(())
}
