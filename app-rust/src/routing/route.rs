use crate::database::pool::DbPool;

use axum::{
    extract::State,
    routing::{get, post},
    Router,
};
use sqlx::query;

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

async fn get_events_count(State(pool): State<DbPool>) -> String {
    match get_current_event_identifier(State(pool)).await {
        Some(id) => id.to_string(),
        None => String::from("No event found or database error"),
    }
}

async fn get_event_by_id(State(pool): State<DbPool>) -> &'static str {
    "get_event_by_id"
}

async fn get_current_event_identifier(State(pool): State<DbPool>) -> Option<u64> {
    let query = "SELECT id FROM status FOR UPDATE";

    match sqlx::query_scalar::<_, u64>(query)
        .fetch_one(&pool)
        .await
    {
        Ok(id) => Some(id),
        Err(e) => {
            eprintln!("Database error: {}", e);
            None
        }
    }
}
