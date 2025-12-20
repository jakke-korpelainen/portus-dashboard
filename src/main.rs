use crate::config::CONFIG;
use axum::{
    response::Redirect,
    routing::{get, Router},
};
use ftail::Ftail;
use log::LevelFilter;
use tower_http::services::ServeDir;

mod config;
mod dashboard;
mod news;
mod residents;
mod templates;
mod transportation;
mod weather;

async fn start_web_server() {
    Ftail::new().console(LevelFilter::Debug).init().unwrap();
    log::info!("Starting portus-dashboard");

    let app = Router::new()
        .route("/", get(|| async { Redirect::permanent("/primary") }))
        .route("/primary", get(dashboard::dashboard_primary))
        .route("/secondary", get(dashboard::dashboard_secondary))
        .nest_service("/assets", ServeDir::new("assets"));

    let server_url = format!("0.0.0.0:{}", CONFIG.port);
    let listener = tokio::net::TcpListener::bind(server_url).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    start_web_server().await;
}
