use std::env;

use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize)]
struct Weather {
    description: String,
}

#[derive(Deserialize)]
struct Main {
    temp: f64,
}

#[derive(Deserialize)]
struct WeatherData {
    weather: Vec<Weather>,
    main: Main,
}

async fn fetch_weather(location: &str) -> Result<(), Error> {
    let api_key = "ec46e688303fc6fda12f9f5c46c59614";
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
        location, api_key
    );

    let response = reqwest::get(&url).await?.json::<WeatherData>().await?;

    println!(
        "The weather in {} is {} \nwith a temperature of {}°C",
        location,
        response.weather[0].description,
        response.main.temp - 273.15 
    );

    Ok(())
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: weather-app <city>");
        std::process::exit(1);
    }

    if let Err(e) = fetch_weather(&args[1]).await {
        eprintln!("Error fetching weather: {}", e);
        eprintln!("Might be invalid city name or network error");
    }
}
