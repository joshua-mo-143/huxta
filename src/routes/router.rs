use crate::routes::webhooks;
use axum::{
    routing::{any, post},
    Router,
};
use reqwest::Client;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub ctx: Client,
}

impl AppState {
    pub fn new(db: PgPool) -> Self {
        Self {
            db,
            ctx: reqwest::Client::new()
        }
    }
}

pub fn init_router(state: AppState) -> Router {
    Router::new()
        .route("/webhooks", any(webhooks::webhook))
        .route("/webhook_history", any(webhooks::webhook_history))
        .route("/redirect", post(webhooks::redirected))
        .with_state(state)
}
