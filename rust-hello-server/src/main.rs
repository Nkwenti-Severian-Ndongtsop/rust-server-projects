use axum::{routing::get, Router};
use tokio::net::TcpListener;
use crate::routers::{get_routes::display, post_routes::upload, get_routes::index};



fn router() -> Router {
    Router::new()
    .route("/hello", get(display))
    .route("/", get(index).post(upload))
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:5555";
    
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Server running on: http://{}", addr);
    
    axum::serve(listener, router()).await.unwrap();
}

mod routers;