use axum::routing::get;
use db::PrismaClient;
use eyre::WrapErr;
use prisma_client_rust as prisma;
use std::{net::SocketAddr, sync::Arc};

mod db;
mod route;

type Db = Arc<PrismaClient>;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let db = new_db("database-url".into())
        .await
        .wrap_err("Failed to create Prisma client")?;
    let db = Arc::new(db);

    let router = new_router(db);

    let address = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&address)
        .serve(router.into_make_service())
        .await
        .wrap_err("Failed to run the server")
}

async fn new_db(url: String) -> Result<db::PrismaClient, prisma::NewClientError> {
    PrismaClient::_builder().with_url(url).build().await
}

fn new_router(db: Arc<PrismaClient>) -> axum::Router {
    use route::*;
    axum::Router::new()
        .route("/health", get(health::get))
        .route("/user", get(user::get).post(user::post))
        .with_state(db)
}
