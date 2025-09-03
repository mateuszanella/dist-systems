use crate::database::pool::DbPool;

use axum::{
    extract::State,
    routing::{get, post},
    Router,
};

pub fn create_router(pool: DbPool) -> Router {
    Router::new()
        .route("/events", post(create_event))
        .route("/events/async", post(create_async_event))
        .route("/events", get(get_events_count))
        .route("/events/:id", get(get_event_by_id))
        .with_state(pool)
}

async fn create_event(State(pool): State<DbPool>) -> &'static str {
    "create_event"
}

async fn create_async_event(State(pool): State<DbPool>) -> &'static str {
    "create_async_event"
}

async fn get_events_count(State(pool): State<DbPool>) -> &'static str {
    "get_events_count"
}

async fn get_event_by_id(State(pool): State<DbPool>) -> &'static str {
    "get_event_by_id"
}

async fn get_current_event_identifier(State(pool): State<DbPool>) -> u64 {
    0
}
