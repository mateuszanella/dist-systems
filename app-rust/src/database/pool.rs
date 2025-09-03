use sqlx::{MySql, Pool, MySqlPool};
use std::time::Duration;

pub type DbPool = Pool<MySql>;

pub async fn create_pool() -> Result<DbPool, sqlx::Error> {
    let connection_options = sqlx::mysql::MySqlConnectOptions::new()
        .host("mysql")
        .port(3306)
        .username("root")
        .password("123123123")
        .database("prod");

    let pool = MySqlPool::connect_lazy_with(connection_options);

    // Test connection with retries
    let mut retries = 0;
    loop {
        match sqlx::query("SELECT 1").fetch_one(&pool).await {
            Ok(_) => {
                log::info!("Database connection successful");
                break;
            }
            Err(e) => {
                if retries >= 5 {
                    log::error!("Failed to connect to database after 5 retries: {}", e);
                    return Err(e);
                }
                log::warn!("Database connection failed, retrying in 5 seconds: {}", e);
                tokio::time::sleep(Duration::from_secs(5)).await;
                retries += 1;
            }
        }
    }

    Ok(pool)
}
