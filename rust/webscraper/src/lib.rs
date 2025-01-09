use reqwest;
use anyhow::{ Result };

pub fn run() -> () {
    let url: String = String::from( "https://google.com" );
    let html: String = match get_html( url ) {
        Ok(html) => html,
        Err(err) => panic!("Error: {}", err),
    };

    println!("{}", html);
}

fn get_html( url: String, ) -> Result<String> {
    let response = reqwest::blocking::get( &url );
    let html_content = response.unwrap().text().unwrap();

    Ok(html_content)
}
