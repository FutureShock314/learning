// use std::collections::HashMap;
use reqwest::{ self, header::HeaderMap };
// use tokio;
// use serde;
// use dotenv;

#[tokio::main]
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    // let lon = "-2.538567";
    let lon = "-2.53";
    // let lat = "51.425403";
    let lat = "51.42";
    let key = dotenv::var("API_KEY").unwrap();
    let exclude = "daily";

    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert( "content-type", "application/json".parse().unwrap() );

    let url = format!( "https://api.openweathermap.org/data/3.0/onecall?lat={}&lon={}&exclude={}&units=metric&appid={}", lat, lon, exclude, key );
    // println!("{:?}", url);

    let response = client.get( url )
        .headers( headers )
        .send()
        .await?;
    // println!("{:?}", response);

    // let response_json = response.json::<HashMap<String, String>>().await?;
    let response_json: serde_json::Value = response.json().await?;

    // println!("{:?}", response_json);
    println!("{:?}", response_json["hourly"][1]);

    Ok(())
}