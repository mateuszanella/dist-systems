use crate::database::{operations, pool::DbPool};
use std::time::Duration;

pub async fn run_worker(pool: DbPool) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Worker started");

    loop {
        match operations::process_event(&pool).await {
            Ok(true) => {
                // Successfully processed an event, continue immediately
                continue;
            }
            Ok(false) => {
                // No events to process, wait before trying again
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
            Err(e) => {
                log::error!("Worker error: {}", e);
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
}
