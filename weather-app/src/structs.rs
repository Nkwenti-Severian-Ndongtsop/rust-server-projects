use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Weather {
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct Main {
    pub temp: f32,
}

#[derive(Debug, Deserialize)]
pub struct WeatherData {
    pub weather: Vec<Weather>,
    pub main: Main,
}

#[derive(Serialize)]
pub struct WeatherResponse {
    pub city: String,
    pub temperature: f32,
    pub description: String,
}
