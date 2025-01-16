mod scraper;

pub fn run() -> () {
    let url: &str = "https://google.com";
    
    let scraper = scraper::Scraper::new( url );

    let elements_h1: Vec<String> = scraper.find_and_return_text(&"h1").expect("bad thign idk");

    println!("{:?}", elements_h1)
}

pub fn print_type<T>(_: &T) { 
    println!("{:?}", std::any::type_name::<T>());
}