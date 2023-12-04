use scraper::html::Select;
use std::collections::HashMap;

use scraper::{Html, Selector};

pub struct RadiataScraper {
    url: String,
    page: Html,
}

impl RadiataScraper {

    pub fn new(url: String) -> RadiataScraper {
        let response = reqwest::blocking::
            get(url.clone()).unwrap()
            .text().unwrap();
        let document = Html::parse_document(&response); 
        
        RadiataScraper { 
            url, 
            page: document 
        }
    }

    pub fn get_url(&self) -> &String {
        &self.url
    }

    pub fn set_url(&mut self, new_url: String) {
        self.url = new_url;
    }

    pub fn get_page(&self) -> &Html {
        &self.page
    }

    fn set_page(&mut self, new_page: Html) {
        self.page = new_page;
    }

    fn update_page(&mut self, new_page_url: String) {
        let response = reqwest::blocking
            ::get(new_page_url.clone()).unwrap()
            .text().unwrap();
        let new_document_page = Html::parse_document(&response);
        self.page = new_document_page;
    }

    pub fn get_list_characters(&self, html_tag: Option<&str>, html_class: Option<&str>) -> HashMap<String, String> {

        let html_tag = html_tag.unwrap_or("li");
        let html_class = html_class.unwrap_or(".category-page__member");

        let selector_string = html_tag.to_owned() + html_class;

        let selector = Selector::parse(&selector_string.to_owned()).unwrap();

        let html_characters_list = self.page.select(&selector);

        let mut character_list : HashMap<String, String> = HashMap::new();

        for html_character in html_characters_list {
            let link = html_character
                .select(&Selector::parse("a").unwrap())
                .next()
                .map(|a| {
                    let href = a.value().attr("href").unwrap().to_string();
                    let text = a.value().attr("title").unwrap().to_string();
                    (href, text)
                });
            
            if let Some((href, text)) = link {
                character_list.insert(href, text);
            }
        }
        character_list
        
    }
                                        ///wiki/Goo   main            .page__main
    pub fn get_character_info(&mut self, url: String, html_tag: &str, html_class: &str) {
        println!("Visitando: {url}");

        self.update_page(url);

        let selector_string = html_tag.to_string() + html_class;

        let selector = Selector::parse(&selector_string).unwrap();

        let html_data = self.page.select(&selector);

        for html_character in html_data {
            let name = html_character
                .select(&Selector::parse("span.mw-page-title-main").unwrap())
                .next()
                .map(|span| span.text().collect::<String>()).unwrap();
            
            //TO DO: Obtencion de Path y RecruimentInfo.
            let img = html_character
                .select(&scraper::Selector::parse(r#"td[colspan="2"] a.image"#).unwrap())
                .next()
                .and_then(|a| a.value().attr("href"))
                .map(str::to_owned).unwrap();

            // let 
            println!("Nombre: {:?}", name);
            println!("IMG: {:?}", img);

        }
    }

}