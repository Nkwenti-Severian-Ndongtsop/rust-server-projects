use crate::routers::{get_routes::display, get_routes::index, post_routes::upload};
use axum::{routing::get, Router};
use tokio::net::TcpListener;

fn router() -> Router {
    Router::new()
        .route("/hello", get(display))
        .route("/", get(index).post(upload))
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8888";

    let listener = TcpListener::bind(addr).await.expect("Failed to read address");
    println!("Server running on: http://{}", addr);

    axum::serve(listener, router()).await.unwrap();
}

mod routers;