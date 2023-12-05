use sqlx::PgPool;
use std::net::SocketAddr;
use webhooks::{init_router, AppState};

pub struct CustomService {
    db: PgPool,
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] db: PgPool,
) -> Result<CustomService, shuttle_runtime::Error> {
    Ok(CustomService { db })
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for CustomService {
    async fn bind(mut self, addr: SocketAddr) -> Result<(), shuttle_runtime::Error> {
        let state = AppState { db: self.db, ctx: reqwest::Client::new() };
        let router = init_router(state);

        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        axum::serve(listener, router).await.unwrap();
        Ok(())
    }
}
