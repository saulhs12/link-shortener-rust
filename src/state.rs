use crate::models::link::Link;
use crate::repository::{
    AuthPostgresRepository, LinkPostgresRepository, StatisticsPostgresRepository,
};
use crate::settings::Settings;
use arc_swap::ArcSwap;
use metrics_exporter_prometheus::PrometheusHandle;
use sqlx::{Pool, Postgres};
use std::sync::Arc;

pub struct ApplicationState {
    pub settings: ArcSwap<Settings>,
    pub db_connection: ArcSwap<Pool<Postgres>>,
    pub link_repository: Arc<LinkPostgresRepository>,
    pub statistics_repository: Arc<StatisticsPostgresRepository>,
    pub auth_repository: Arc<AuthPostgresRepository>,
}

impl ApplicationState {
    pub fn new(settings: &Settings, pool: Pool<Postgres>) -> anyhow::Result<Self> {
        Ok(Self {
            settings: ArcSwap::new(Arc::new((*settings).clone())),
            db_connection: ArcSwap::new(Arc::new(pool.clone())),
            link_repository: Arc::new(LinkPostgresRepository::new(pool.clone())),
            statistics_repository: Arc::new(StatisticsPostgresRepository::new(pool.clone())),
            auth_repository: Arc::new(AuthPostgresRepository::new(pool.clone())),
        })
    }
}
