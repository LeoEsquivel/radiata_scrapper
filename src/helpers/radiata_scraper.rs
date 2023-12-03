use scraper::Html;

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

}