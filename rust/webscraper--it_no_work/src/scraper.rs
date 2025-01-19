use reqwest::{ self, header::{ HeaderMap,
    // HeaderValue, USER_AGENT
} };
use anyhow::{ Result };
use scraper;

pub struct Scraper {
    url: String,
    html: String,
    parsed_html: scraper::Html,
    headers: HeaderMap,
}

impl Scraper {
    pub fn new( url: &str ) -> Scraper {
        let mut scraper = Scraper {
            url: url.to_string(),
            html: String::new(),
            parsed_html: scraper::Html::new_document(),
            headers: HeaderMap::new(),
        };
        scraper.parsed_html = scraper.get_html().unwrap();
        scraper
    }

    pub fn set_headers( &mut self, headers: HeaderMap ) {
        self.headers = headers;
    }

    pub fn html( &self ) -> &str {
        &self.html
    }

    pub fn url( &self ) -> &str {
        &self.url
    }

    fn get_html( &mut self ) -> Result<scraper::Html> {
        let client = reqwest::blocking::Client::builder()
            .default_headers( self.headers.clone() )
            .build()?;

        let response = client.get( &self.url ).send()?;

        if response.status().is_success() {
            let html_content = response.text().unwrap();
            let parsed_html = scraper::Html::parse_document( &html_content );
            self.html = html_content;

            Ok(parsed_html)
        } else {
            panic!( "Error fetching HMTL: {}", response.status() )
        }
    }

    pub fn find_and_return_text( &self, css_selector: &str ) -> Result<Vec<String>> {
        let selector = scraper::Selector::parse( css_selector ).unwrap();

        let elements = self.parsed_html.select( &selector );
        let mut element_vec = vec![];

        for el in elements {
            element_vec.push( el.text().collect::<String>() );
        };

        Ok(element_vec)
    }
}
