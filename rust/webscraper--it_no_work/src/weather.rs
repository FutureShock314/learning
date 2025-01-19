use crate::scraper::Scraper;
use reqwest::header::{ HeaderMap, HeaderValue, USER_AGENT };

pub fn get_weather() {
    let url: &str = "https://weather.metoffice.gov.uk/forecast/gcnhtfzhd#";
    let mut scraper = Scraper::new( url );
    let agent = "Mozilla/5.0 (iPad; CPU OS 12_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148";
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert( USER_AGENT, HeaderValue::from_static( &agent ) );

    scraper.set_headers( headers );

    println!("--DEBUG-- url = {}", scraper.url());
    println!("--DEBUG-- html = {}", scraper.html());

    // let times = scraper.find_and_return_text( "td[headers^=\"d0Temp d0t\"]" ).unwrap();
    let times = scraper.find_and_return_text( r#"th[id^="d0t"]"# ).unwrap();
    println!( "{:?}", times )
}
