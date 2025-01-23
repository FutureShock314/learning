// use std::collections::HashMap;
use reqwest::{ self, header::HeaderMap };
use chrono::DateTime;
use serde_json::Value;
// use serde::Deserialize;
// use tokio;
// use dotenv;
use num_traits::cast::ToPrimitive; // for `to_i64` on `f64` type

mod color;

#[tokio::main]
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {

    /*
    *
    * FETCH DATA
    *
    */

    let lon = "-2.53";
    let lat = "51.42";
    let key = dotenv::var("API_KEY").unwrap();
    let exclude = "daily,minutely";

    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert( "content-type", "application/json".parse().unwrap() );

    let url = format!(
        "https://api.openweathermap.org/data/3.0/onecall?lat={}&lon={}&exclude={}&cnt=3&units=metric&appid={}",
        lat, lon, exclude, key );

    let response = client.get( url )
        .headers( headers )
        .send()
        .await?;

    // let response_json = response.json::<HashMap<String, String>>().await?;
    let response_json: Value = response.json().await?;
    // println!("{}", response_json["hourly"].as_array().unwrap()[1]);

    /*
    *
    * PRESENT DATA
    *
    */

    let current = &response_json["current"];
    let hourly = &response_json["hourly"].as_array().unwrap();
    // println!( "{:#}", current );

    // CURRENT
    let current_ts = current["dt"].as_i64().unwrap();
    let current_dt = DateTime::from_timestamp( current_ts, 0 )
        .expect( "Invalid UNIX Timestamp! ( `current_ts` )" );

    println!( "" );

    println!( "Current Weather ( {:05} ):", current_dt.format( "%H:%M" ) );
    println!(
        "  {temp:05}°C  |  {weather_desc}",
        temp=current["temp"].as_f64().unwrap(),
        weather_desc=current["weather"][0]["description"].as_str().unwrap(),
    );
    println!( "" );

    // println!("{:#}", hourly[1]);

    let time_col = color::Color::new( 000, 255, 200 );
    let like_col = color::Color::new( 255, 150, 150 );
    let prob_col = color::Color::new( 000, 175, 255 );

    for index in 0..6 {
        let hour = &hourly[index];
        let hourly_ts = hourly[index]["dt"].as_i64().unwrap();
        let hourly_dt = DateTime::from_timestamp( hourly_ts, 0 )
            .expect( "Invalid UNIX Timestamp! ( `hourly_ts` )" );

        println!(
            "{time}:  {temp:05.2}°C -> Feels like {like}  |  {prob}% Chance of rain",
            time = color::colored(
                &time_col,
                &format!( "{}", hourly_dt.format( "%Y-%m-%d %H:%M" ) )
            ),
            temp = hour["temp"].as_f64().unwrap(),
            like = color::colored(
                &like_col,
                &format!( "{:05.2}°C", hour["feels_like"].as_f64().unwrap() )
            ),
            prob = color::colored(
                &prob_col,
                &format!(
                    "{:>3}",
                    ( hour["pop"].as_f64().unwrap() * 100.0 ).to_i64().unwrap()
                )
            ),
        );
    }

    println!( "" );

    Ok(())
}