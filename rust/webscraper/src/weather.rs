use crate::scraper::Scraper;

pub fn get_weather() {
    let url: &str = "https://weather.metoffice.gov.uk/forecast/gcnhtfzhd#";
    let scraper = Scraper::new( url );

    // let times = scraper.find_and_return_text( "td[headers^=\"d0Temp d0t\"]" ).unwrap();
    let times = scraper.find_and_return_text( "td" ).unwrap();
    println!( "{:?}", times )
}
