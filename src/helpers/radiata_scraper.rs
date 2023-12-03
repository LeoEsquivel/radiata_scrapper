use scraper::Html;

pub struct RadiataScraper {
    url: String,
    doc: Html,
}

impl RadiataScraper {

    pub fn new(url: String) -> RadiataScraper {
        let response = reqwest::blocking::
            get(url.clone()).unwrap()
            .text().unwrap();
        let document = Html::parse_document(&response); 
        
        RadiataScraper { 
            url, 
            doc: document 
        }
    }
}