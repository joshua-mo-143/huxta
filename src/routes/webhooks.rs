use crate::routes::router::AppState;
use crate::webhooks::Webhook;
use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
    Json,
};

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
