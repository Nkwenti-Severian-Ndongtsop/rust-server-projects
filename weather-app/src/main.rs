use std::env;

use reqwest::Error;
use structs::WeatherData;

mod structs;

async fn fetch_weather(location: &str) -> Result<(), Error> {
    let api_key = "ec46e688303fc6fda12f9f5c46c59614";
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
        location, api_key
    );

    let response = reqwest::get(&url).await?.json::<WeatherData>().await?;

    println!(
        "The weather in {} is {} \nwith a temperature of {}°C\n",
        location,
        response.weather[0].description,
        response.main.temp - 273.15
    );

    Ok(())
}

async fn geolocation() -> Result<(), Error> {
    let url = "http://ip-api.com/json/";
    let response = reqwest::get(url)
        .await?
        .json::<structs::GeoLocation>()
        .await?;
    println!(
        "Your Hosted on: {}
    With a Timezone of: {}
    In the City: {}
    Your ISP is: {}",
        response.country, response.timezone, response.city, response.isp
    );

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "http://ip-api.com/json/";

    let default_loc = reqwest::get(url)
        .await?
        .json::<structs::GeoLocation>()
        .await?;
    let mut args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        let location = default_loc.country.clone();
        args.push(location);
    } else {
        eprintln!("Usage: weather-app <city>");
        std::process::exit(1);
    }

    if let Err(e) = fetch_weather(&args[1]).await {
        eprintln!("Error fetching weather: {}", e);
        eprintln!("Might be invalid city name or network error");
    }

    if let Err(e) = geolocation().await {
        eprintln!("Error fetching your data: {}", e);
        eprintln!("Might be network issues");
    }

    Ok(())
}
