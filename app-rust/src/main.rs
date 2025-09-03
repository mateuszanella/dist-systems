use std::net::SocketAddr;

mod routing;

mod database;

use database::pool::create_pool;
use routing::route::create_router;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt::init();

    let pool = create_pool().await.expect("Failed to create database pool");

    let app = create_router(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    println!("listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
