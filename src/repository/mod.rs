pub mod auth;
pub mod link;
pub mod statistics;

use sqlx::PgPool;

pub struct LinkPostgresRepository {
    pub pool: PgPool,
}

impl LinkPostgresRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

pub struct StatisticsPostgresRepository {
    pub pool: PgPool,
}

impl StatisticsPostgresRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

pub struct AuthPostgresRepository {
    pub pool: PgPool,
}

impl AuthPostgresRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
