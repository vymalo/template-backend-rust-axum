use std::fmt::Debug;
use derive_builder::Builder;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::AsyncPgConnection;

#[derive(Clone, Builder)]
pub struct DbState {
    pub pool: Pool<AsyncPgConnection>,
}

impl Debug for DbState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("_db").finish()
    }
}