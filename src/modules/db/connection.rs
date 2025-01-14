use crate::modules::db::migrations::run_migrations;
use crate::modules::env::env::EnvConfig;
use anyhow::Result;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;

pub async fn get_connection(
    EnvConfig {
        db_url,
        db_max_thread_pool,
        ..
    }: EnvConfig,
) -> Result<Pool<AsyncPgConnection>> {
    let manager = AsyncDieselConnectionManager::new(&db_url);
    let pool = Pool::builder(manager)
        .max_size(db_max_thread_pool)
        .build()?;

    run_migrations(db_url).await?;
    Ok(pool)
}