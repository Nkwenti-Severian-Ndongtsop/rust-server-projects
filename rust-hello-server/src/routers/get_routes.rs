use axum::response::Html;

pub async fn display() -> &'static str {
    "Hello, World!"
}

pub async fn index() -> Html<&'static str> {
    Html(std::include_str!("../../public/index.html"))
}
