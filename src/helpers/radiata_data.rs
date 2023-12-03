use std::collections::HashMap;

use crate::models::character::Character;

pub struct RadiataData {
    character_list: HashMap<String, Character>,
}


impl RadiataData {
    pub fn new() -> RadiataData {
        RadiataData { character_list: HashMap::new() }
    }

    pub fn add_character_to_vec(&mut self, character: Character) {
        let name = character.get_name().clone();
        if !self.character_list.contains_key(&name) {
            self.character_list.insert(name, character);
        }
    }
}
