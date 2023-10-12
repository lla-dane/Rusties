use serde::{Deserialize, Serialize};
use std::*;
use reqwest;

#[derive(Debug, Serialize, Deserialize)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    sys: Sys,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Weather {
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Main {
    temp: f64,
    pressure: f64,
    humidity: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Sys {
    country: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Wind {
    speed: f64,
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

     let args: Vec<String> = env::args().collect();
     if args.len() < 2 {
           panic!("Not enough arguments!");
     }

    let api_key = "804f3146cf294dab25e3ee2c528f3425";
    let city = args[1].clone();



    let response : WeatherResponse= reqwest::Client::new()
        .get(format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid={}", city, api_key))
        .send()
        .await?
        .json()
        .await?;

        // let mut file = File::create("weather.json")?;
        // file.write_all(&response.as_bytes())?;
    
    println!("Place: {}", &response.name);
    println!("Country: {}", &response.sys.country);
    println!("Sky: {}", &response.weather[0].description);
    println!("Temperature: {}", &response.main.temp - 273.13);
    println!("Pressure: {}", &response.main.pressure);
    println!("Humidity: {}%", &response.main.humidity);
    println!("Wind_Speed: {}",&response.wind.speed); 

    Ok(())
}