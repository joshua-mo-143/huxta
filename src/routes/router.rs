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

pub fn init_router(state: AppState) -> Router {
    Router::new()
        .route("/webhooks", any(webhooks::webhook))
        .route("/redirect", post(webhooks::redirected))
        .with_state(state)
}
