mod models;
mod helpers;
use models::character::Character;

use crate::helpers::radiata_scraper::RadiataScraper;


fn main() {
    let url = "https://radiata.fandom.com";
    let mut radiata_scraper = RadiataScraper::new(url.to_string()+"/wiki/Category:Recruitable_Characters");

    let list = radiata_scraper.get_list_characters(Some("li"),Some(".category-page__member"));
    let mut character_list: Vec<Character> = Vec::new();
    for (link, _) in list {

        if link.contains("Category") {
            continue;
        }
        let character_page = url.to_string() + &link;
        
        let character = radiata_scraper.get_character_info(character_page, "main", ".page__main");
        character_list.push(character);
   }

   for c in character_list {
        println!("{:?}", c.get_name());
   }

}

