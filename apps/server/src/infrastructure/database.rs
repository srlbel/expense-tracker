use sqlx::PgPool;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use std::time::Duration;

pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    let options = database_url
        .parse::<PgConnectOptions>()?
        .application_name("Expense-Tracker")
        .statement_cache_capacity(0);

    PgPoolOptions::new()
        .max_connections(5)
        .min_connections(0)
        .acquire_timeout(Duration::from_secs(3))
        .connect_with(options)
        .await
}
