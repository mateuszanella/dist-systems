use std::net::SocketAddr;

mod routing;

use routing::route::create_router;

#[tokio::main]
async fn main() {
    let app = create_router();

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    println!("listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
