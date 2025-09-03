use crate::database::pool::DbPool;
use crate::database::operations;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde_json::{json, Value};

pub fn create_router(pool: DbPool) -> Router {
    Router::new()
        .route("/events", post(create_event))
        .route("/events/async", post(create_async_event))
        .route("/events", get(get_events_count))
        .route("/events/:id", get(get_event_by_id))
        .with_state(pool)
}

async fn create_event(State(pool): State<DbPool>) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match operations::create_sync_event(&pool).await {
        Ok(event) => Ok(Json(json!(event))),
        Err(e) => {
            log::error!("Failed to create sync event: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()}))
            ))
        }
    }
}

async fn create_async_event(State(pool): State<DbPool>) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match operations::create_async_event(&pool).await {
        Ok(event) => Ok(Json(json!(event))),
        Err(e) => {
            log::error!("Failed to create async event: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()}))
            ))
        }
    }
}

async fn get_events_count(State(pool): State<DbPool>) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match operations::get_event_count(&pool).await {
        Ok(count) => Ok(Json(json!({"count": count}))),
        Err(e) => {
            log::error!("Failed to get event count: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()}))
            ))
        }
    }
}

async fn get_event_by_id(
    State(pool): State<DbPool>,
    Path(id): Path<i32>
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match operations::get_event_by_id(&pool, id).await {
        Ok(Some(event)) => Ok(Json(json!(event))),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "event not found"}))
        )),
        Err(e) => {
            log::error!("Failed to get event by id {}: {}", id, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()}))
            ))
        }
    }
}
