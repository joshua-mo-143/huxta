use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
    routing::{any, get, post},
    Json, Router,
};
use sqlx::PgPool;
use reqwest::Client;

mod webhooks;

use webhooks::Webhook;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub ctx: Client 
}

pub fn init_router(state: AppState) -> Router {
    Router::new()
        .route("/webhooks", any(webhooks))
        .route("/redirect", post(redirected))
        .with_state(state)
}

async fn redirected(
    Webhook(request): Webhook
    )  -> impl IntoResponse {
    Json(request)
}

async fn webhooks(State(state): State<AppState>, Webhook(request): Webhook) -> impl IntoResponse {
    println!("{:?}", request.to_owned());
    sqlx::query(
        "INSERT INTO requests (
            method,
            origin,
            version,
            headers,
            request_body
            ) VALUES (
                $1, $2, $3, $4, $5
                )",
    )
    .bind(request.method.clone())
    .bind(request.origin.clone())
    .bind(request.version.clone())
    .bind(request.headers.clone())
    .bind(request.body.clone())
    .execute(&state.db)
    .await
    .unwrap();

    Redirect::temporary("/redirect")
}
