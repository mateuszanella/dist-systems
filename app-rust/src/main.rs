mod database;
mod models;
mod routing;
mod worker;

use database::pool::create_pool;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    let mode = args.get(1).map(|s| s.as_str()).unwrap_or("server");

    let pool = create_pool().await?;

    match mode {
        "worker" => {
            log::info!("Starting worker mode");
            worker::run_worker(pool).await?;
        }
        "server" | _ => {
            log::info!("Starting server mode");
            let app = routing::route::create_router(pool);
            let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
            log::info!("Server listening on 0.0.0.0:8080");
            axum::serve(listener, app).await?;
        }
    }

    Ok(())
}
