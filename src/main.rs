mod api;
mod compiler;
mod db;
mod lessons;

use anyhow::Result;
use axum::{
    response::Html,
    routing::get,
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;

use crate::api::{router as api_router, AppState};
use crate::db::Database;
use crate::lessons::built_in_lessons;

const INDEX_HTML: &str = include_str!("static/index.html");

#[tokio::main]
async fn main() -> Result<()> {
    let db = Database::open()?;
    let lessons = built_in_lessons();

    let state = Arc::new(AppState {
        db: Mutex::new(db),
        lessons,
    });

    let app = Router::new()
        .route("/", get(|| async { Html(INDEX_HTML) }))
        .merge(api_router(state))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8765));
    println!("rusticize running at http://{}", addr);
    println!("NEMESIS ENGINE Rust Learning Platform");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
