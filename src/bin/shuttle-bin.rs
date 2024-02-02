use huxta::routes::router::{init_router, AppState};
use sqlx::PgPool;
use std::net::SocketAddr;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] db: PgPool,
) -> shuttle_axum::ShuttleAxum {
        sqlx::migrate!().run(&db).await.expect("Failed to run migrations");

        let state = AppState::new(db);

        let router = init_router(state);

        Ok(router.into())
}
