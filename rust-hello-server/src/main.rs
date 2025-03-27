use crate::routers::{
    get_routes::display, get_routes::index, post_routes::send_file, post_routes::upload,
};
use axum::{
    routing::{get, post},
    Router,
};
use routers::post_routes::compress_file;
use tokio::net::TcpListener;

fn router() -> Router {
    Router::new()
        .route("/hello", get(display))
        .route("/", get(index).post(upload))
        .route("/upload", post(send_file))
        .route("/compress", post(compress_file))
}

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:8000";

    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to read address");
    println!("Server running on: http://{}", addr);

    axum::serve(listener, router()).await.unwrap();
}

mod routers;
