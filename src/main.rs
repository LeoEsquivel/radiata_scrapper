mod models;
mod helpers;
use crate::helpers::radiata_scraper::RadiataScraper;


fn main() {
    let url = "https://radiata.fandom.com/wiki/Category:Recruitable_Characters";
    let mut scrap = RadiataScraper::new(url.to_string());

    println!("URL Actual: {}", scrap.get_url());

    let list = scrap.get_list_characters(Some("li"),Some(".category-page__member"));

    for (link, nombre) in list {
        println!("{nombre}: {link}")
    }

}