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
        "\n🌤️ The weather in {} is {} \n🌡️ Temperature: {:.2}°C",
        location,
        response.weather[0].description,
        response.main.temp - 273.15 // Convert from Kelvin to Celsius
    );

    Ok(())
}

async fn geolocation() -> Result<String, Error> {
    let url = "http://ip-api.com/json/";
    let response = reqwest::get(url).await?.json::<structs::GeoLocation>().await?;

    println!(
        "\n📍 Location Info:
    🌍 Country: {}
    🏙️ City: {}
    🕰️ Timezone: {}
    📡 ISP: {}\n",
        response.country, response.city, response.timezone, response.isp
    );

    Ok(response.city)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Get city argument from CLI
    let city = env::args().nth(1);

    // If no city is provided, auto-detect location
    let location = match city {
        Some(city_name) => city_name,
        None => {
            println!("No city provided. Detecting location...");
            geolocation().await.unwrap_or_else(|_| {
                eprintln!("Failed to detect location. Please enter a city manually.");
                std::process::exit(1);
            })
        }
    };

    // Fetch weather
    if let Err(e) = fetch_weather(&location).await {
        eprintln!("❌ Error fetching weather: {}", e);
        eprintln!("⚠️ Might be an invalid city name or network error.");
    }

    Ok(())
}
