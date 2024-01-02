use std::path::PathBuf;

use csv::Writer;

use crate::models::character::Character;

pub struct RadiataData {
}


impl RadiataData {

    pub fn create_csv(&self, character_data: Vec<Character>) {
        // let path = Path::new("../../data/radiata_data.csv");
        let mut path = self.get_data_path();
        path.push("radiata_data.csv");
        println!("Escribiendo archivo en: {:?}", path.to_str());

        let mut writer = Writer::from_path(path).unwrap();

        writer.write_record(&["name", "path", "recruitment", "images"]).unwrap();

        for character in character_data {
            
            let name = character.get_name();
            let path = character.get_path();
            let recruitment = character.get_recruitment();
            let image = character.get_image();
            
            println!("Guardando a: {}", name);
            writer.write_record(&[name, &path.to_string(), &recruitment.to_string(), image]).unwrap();
        }
        writer.flush().unwrap();
    }

    fn get_data_path(&self) -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("data");
        path
    }
}
