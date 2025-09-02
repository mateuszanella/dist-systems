use axum::{extract::Path, routing::{get, post}, Router};

async fn get_events() -> &'static str {
    "ok"
}

async fn get_event_by_id(Path(id): Path<u64>) -> &'static str {
    "ok"
}

async fn create_event() -> &'static str {
    "ok"
}

async fn create_event_async() -> &'static str {
    "ok"
}

pub fn create_router() -> Router {
    Router::new()
        .route("/events", get(get_events))
        .route("/events/:id", get(get_event_by_id))
        .route("/events", post(create_event))
        .route("/events/async", post(create_event_async))
}
