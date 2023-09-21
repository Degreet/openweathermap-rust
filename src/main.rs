use dotenv;
use reqwest;

use clap::{Parser};
use serde::Deserialize;

#[derive(Parser)]
#[command(name = "forecast")]
#[command(about = "Weather in your terminal", long_about = None)]
struct Args {
    #[arg(long, default_value_t = -41.2)]
    lat: f32,

    #[arg(long, default_value_t = 174.7)]
    lon: f32,
}

#[derive(Deserialize, Debug)]
struct WeatherInfo {
    description: String,
}

#[derive(Deserialize, Debug)]
struct MainInfo {
    temp: f32,
    feels_like: f32,
}

#[derive(Deserialize, Debug)]
struct WeatherApiResponse {
    weather: Vec<WeatherInfo>,
    main: MainInfo,
}

fn main() -> Result<(), reqwest::Error> {
    dotenv::dotenv().unwrap();

    let args = Args::parse();
    let lat = args.lat;
    let lon = args.lon;

    let api_key = std::env::var("API_KEY").expect("API_KEY is not defined");
    let api_url = format!("https://api.openweathermap.org/data/2.5/weather?lat={lat}&lon={lon}&appid={api_key}&units=metric");

    let response: WeatherApiResponse = reqwest::blocking::get(api_url)?.json()?;
    let weather_description = &response.weather[0].description;
    let weather_info = &response.main;

    println!("It's {} and {}°C. Feels like {}°C", weather_description, weather_info.temp, weather_info.feels_like);

    Ok(())
}
