mod models;
mod helpers;
use crate::helpers::radiata_scraper::RadiataScraper;


fn main() {
    let url = "https://radiata.fandom.com";
    let mut radiata_scraper = RadiataScraper::new(url.to_string()+"/wiki/Category:Recruitable_Characters");

    let list = radiata_scraper.get_list_characters(Some("li"),Some(".category-page__member"));

    for (link, _) in list {

        if link.contains("Category") {
            continue;
        }
        let character_page = url.to_string() + &link;
        
        radiata_scraper.get_character_info(character_page, "main", ".page__main");
        break;
   }

}

