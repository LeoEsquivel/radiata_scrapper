use core::fmt;

pub enum Path {
    Human,
    Fairy,
    Any,
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Path::Human => write!(f, "Human"),
            Path::Fairy => write!(f, "Fairy"),
            Path::Any => write!(f, "Any")

        }
    }
}

pub struct RecruimentInfo {
    requirements: Vec<String>,
    directions: Vec<String>,
}

pub struct Character {
    name        : String,
    path        : Path,
    recruitment : RecruimentInfo,
    affiliation : String,
    image       : String,
}

impl Character {

    pub fn new(name: String, path: Path, affiliation: String, image: String, requirements: Vec<String>, directions: Vec<String>) -> Character {
        let recruitment_info = RecruimentInfo {
            requirements,
            directions,
        };

        Character {
            name,
            path,
            recruitment: recruitment_info,
            affiliation,
            image
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    pub fn get_path(&self) -> &Path {
        &self.path
    }

    pub fn set_path(&mut self, new_path: Path) {
        self.path = new_path
    }

    pub fn get_recruitment(&self) -> &RecruimentInfo {
        &self.recruitment
    }

    pub fn set_recruitment(&mut self, new_recruitment: RecruimentInfo) {
        self.recruitment = new_recruitment
    }

}