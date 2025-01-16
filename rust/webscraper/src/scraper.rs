use reqwest;
use anyhow::{ Result };
use scraper;

pub struct Scraper {
    url: String,
    html: String,
    parsed_html: scraper::Html,
}

impl Scraper {
    pub fn new( url: &str ) -> Scraper {
        let mut scraper = Scraper {
            url: url.to_string(),
            html: String::new(),
            parsed_html: scraper::Html::new_document(),
        };
        scraper.get_html().unwrap();
        scraper
    }

    pub fn html( &self ) -> &str {
        &self.html
    }

    pub fn url( &self ) -> &str {
        &self.url
    }

    fn get_html( &mut self ) -> Result<scraper::Html> {
        let response = reqwest::blocking::get( &self.url );
        let html_content = response.unwrap().text().unwrap();
        let parsed_html = scraper::Html::parse_document( &html_content );
        self.html = html_content;

        Ok(parsed_html)
    }

    pub fn find_and_return_text( &self, css_selector: &str ) -> Result<Vec<String>> {
        let selector = scraper::Selector::parse( css_selector ).unwrap();

        let elements = self.parsed_html.select( &selector );
        let mut element_vec = vec![];

        for el in elements {
            element_vec.push( el.inner_html() );
        };

        Ok(element_vec)
    }
}
