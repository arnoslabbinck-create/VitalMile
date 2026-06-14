use axum::{routing::get, Router};

use super::runsessionapi::get_runsession_handler;

pub fn create_router() -> Router {
    Router::new()
        .route("api/runsession", get(get_runsession_handler))
}