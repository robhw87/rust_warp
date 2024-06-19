use sqlx::{Executor, Error, Pool, Postgres};
use std::sync::Arc;

async fn initialize_pool() -> Result<Arc<sqlx::Pool<Postgres>>, Error> {
    let url = "postgres://postgres:password123@localhost/maindb";
    let pool = sqlx::postgres::PgPool::connect(url)
        .connect_timeout(std::time::Duration::from_secs(5)) // Optional: connection timeout
        .max_connections(max_connections)
        .await?;

    // Set maximum connection limit
    let max_connections = 10; // Adjust as needed

    let pool = Pool::connect_with(Config::new()
        .connect_timeout(std::time::Duration::from_secs(5)) // Optional: connection timeout
        .max_connections(max_connections)
        .build::<Postgres>(Executor::postgres())?, &db_url)
        .await?;

    Ok(Arc::new(pool))
}