mod scraper;
mod weather;

pub fn run() -> () {
    // let url: &str = "https://scrapingcourse.com/ecommerce/";
    
    // let scraper = scraper::Scraper::new( url );

    // let elements_div: Vec<String> = scraper.find_and_return_text(&"div").expect("bad thign idk");

    // println!("{:?}", elements_div);

    weather::get_weather();
}

pub fn print_type<T>(_: &T) {
    println!("{:?}", std::any::type_name::<T>());
}