use axum::{
    extract::{FromRequest, Request},
    http::{version::Version, Method},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map};

pub struct Webhook(pub WebhookRequest);

#[derive(Serialize, Debug, sqlx::FromRow, Clone)]
pub struct WebhookRequest {
    pub method: HttpMethod,
    pub origin: String,
    pub version: HttpVersion,
    pub headers: serde_json::Value,
    pub body: Option<serde_json::Value>,
}

#[async_trait::async_trait]
impl<S> FromRequest<S> for Webhook
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let mut headers = Map::new();

        for (key, value) in req.headers().iter() {
            headers.insert(key.to_string(), json!(value.to_str().unwrap().to_owned()));
        }

        let method = match *req.method() {
            Method::GET => HttpMethod::Get,
            Method::POST => HttpMethod::Post,
            _ => {
                return Err(
                    ("That's not a valid method! Only GET and POST are accepted.").into_response(),
                )
            }
        };

        let version = match req.version() {
            Version::HTTP_10 => HttpVersion::Http1_0,
            Version::HTTP_11 => HttpVersion::Http1_1,
            Version::HTTP_2 => HttpVersion::Http2_0,
            _ => {
                return Err(
                    ("That's not a valid HTTP version! Only 1.0, 1.1 and 2.0 are accepted.")
                        .into_response(),
                )
            }
        };

        let origin = req.uri().to_string();

        let meme = axum::body::to_bytes(req.into_body(), 432532532)
            .await
            .unwrap();

        let body = match serde_json::from_slice::<serde_json::Value>(&meme) {
            Ok(res) => Some(res),
            Err(e) => {
                println!("Error while getting json: {e}");
                None
            }
        };

        let webhook_request = WebhookRequest {
            method,
            origin,
            version,
            headers: serde_json::Value::Object(headers),
            body,
        };

        Ok(Self(webhook_request))
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone)]
#[sqlx(type_name = "http_method")]
pub enum HttpMethod {
    Get,
    Post,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone)]
#[sqlx(type_name = "http_version")]
pub enum HttpVersion {
    Http1_0,
    Http1_1,
    Http2_0,
}
