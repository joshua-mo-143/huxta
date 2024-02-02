use crate::routes::router::AppState;
use crate::webhooks::Webhook;
use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
    Json,
    http::StatusCode,
};
use serde::Serialize;
use crate::webhooks::{HttpMethod, HttpVersion};
use chrono::{DateTime, Utc};

#[derive(sqlx::FromRow, Serialize)]
pub struct WebhookJson {
    id: i32,
    method: HttpMethod,
    origin: String,
    version: HttpVersion,
    headers: serde_json::Value,
    request_body: Option<serde_json::Value>,
    created_at: DateTime<Utc>
}
pub async fn redirected(Webhook(request): Webhook) -> impl IntoResponse {
    Json(request)
}

pub async fn webhook(
    State(state): State<AppState>,
    Webhook(request): Webhook,
) -> impl IntoResponse {
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
    .bind(request.method())
    .bind(request.origin())
    .bind(request.version())
    .bind(request.headers())
    .bind(request.body())
    .execute(&state.db)
    .await
    .unwrap();

    Redirect::temporary("/redirect")
}

pub async fn webhook_history(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let res = match sqlx::query_as::<_, WebhookJson>("SELECT * FROM REQUESTS")
        .fetch_all(&state.db)
        .await {
        Ok(res) => res,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        };

    Ok(Json(res))


}
