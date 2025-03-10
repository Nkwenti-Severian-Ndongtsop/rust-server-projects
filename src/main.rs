use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:5555";

    let response = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(response, router()).await.unwrap()
}

fn router() -> Router {
    Router::new().route("/", get(display))
}

async fn display() -> &'static str {
    "Hello, World!"
}
