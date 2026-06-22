use axum::{routing::get, Router, routing::post};

use crate::api::runsessionapi::post_create_runsession_handler;

use super::runsessionapi::get_runsession_handler;


pub fn create_router() -> Router {
    Router::new()
        .route("/api/runsession", get(get_runsession_handler))
        .route("/api/runsession", post(post_create_runsession_handler))
    }