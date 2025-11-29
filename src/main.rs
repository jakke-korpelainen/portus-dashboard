use axum::{
    response::Redirect,
    routing::{get, Router},
};
use dotenv::dotenv;
use tower_http::services::ServeDir;

mod config;
mod dashboard;
mod news;
mod residents;
mod templates;
mod transportation;
mod weather;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenv().ok();
    start_web_server().await;
}

async fn start_web_server() {
    const WEB_SERVER_PORT: u16 = 3000;

    let app = Router::new()
        .route("/", get(|| async { Redirect::permanent("/primary") }))
        .route("/primary", get(dashboard::dashboard_primary))
        .route("/secondary", get(dashboard::dashboard_secondary))
        .nest_service("/assets", ServeDir::new("assets"));

    let server_url = format!("0.0.0.0:{}", WEB_SERVER_PORT);
    let listener = tokio::net::TcpListener::bind(server_url).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
