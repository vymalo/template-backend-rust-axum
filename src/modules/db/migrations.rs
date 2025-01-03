use app_models::Result;
use diesel::Connection;
use diesel_async::async_connection_wrapper::AsyncConnectionWrapper;
use diesel_async::AsyncPgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[inline]
pub async fn run_migrations(database_url: String) -> Result<()> {
    tokio::task::spawn_blocking(move || {
        let mut conn = AsyncConnectionWrapper::<AsyncPgConnection>::establish(&database_url)
            .expect("Failed to establish connection");
        conn.run_pending_migrations(MIGRATIONS)
            .expect("Failed to run migrations");
    })
    .await?;
    Ok(())
}