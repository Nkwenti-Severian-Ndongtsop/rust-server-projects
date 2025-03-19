use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};

mod structs;

#[axum::debug_handler]
async fn weather(Path(location): Path<String>) -> Result<impl IntoResponse, Response> {
    let api_key = "bf053a6405289e6477153f723c063be0";
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
        location, api_key
    );
    let response = reqwest::get(&url)
        .await
        .expect("Couldn't Get the data")
        .json::<structs::WeatherData>()
        .await
        .map_err(|_| {
            (
                axum::http::StatusCode::NOT_FOUND,
                "City not found".to_string(),
            )
                .into_response()
        });

    match response {
        Ok(data) => {
            println!(
                "The weather in {:?} is {:?} \nwith a temperature of {:?}°C",
                location, data.weather[0].description, data.main.temp
            );

            Ok(Json(structs::WeatherResponse {
                city: location,
                temperature: data.main.temp,
                description: data.weather[0].description.clone(),
            })
            .into_response())
        }
        Err(_) => Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "City not found".to_string(),
        )
            .into_response()),
    }
}

fn router() -> Router {
    Router::new().route("/weather/{location}", get(weather))
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind port");

    println!("Server is running on: http://{}", addr);

    axum::serve(listener, router()).await.expect("server error")
}
