use axum::{
    extract::Path, response::IntoResponse, routing::get, Json, Router
};

mod structs;

async fn weather(Path(city): Path<String>) -> impl IntoResponse {
    let api_key = "bf053a6405289e6477153f723c063be0";
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
        city, api_key
    );
    let response = reqwest::get(&url)
    .await
    .expect("Couldn't Get the data")
    .json::<structs::WeatherData>()
    .await
    .expect("Couldn't parse the data");

    println!("The weather in {:?} is {:?} \nwith a temperature of {:?}°C", city, response.weather[0].description, response.main.temp);

    Json(structs::WeatherResponse {
        city: city.clone(),
        temperature: response.main.temp,
        description: response.weather[0].description.clone(),
    })
}

fn router() -> Router {
    Router::new().route("/weather/{city}", get(weather))
}

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:8080";

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Invalid address");

    println!("Server is running on: http://{}", addr);

    axum::serve(listener, router())
        .await
        .expect("Invalid server");
}
