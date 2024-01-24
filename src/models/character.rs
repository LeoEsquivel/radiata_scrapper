use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct RecruitmentInfo {
    pub requirements: Vec<String>,
    pub directions: Vec<String>,
}

impl fmt::Display for RecruitmentInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let requirements = self.requirements.join(", ");
        let directions = self.directions.join(", ");
        write!(f, "Requirements: {}, Directions: {}", requirements, directions)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Character {
    pub name        : String,
    pub path        : Path,
    pub recruitment : RecruitmentInfo,
    pub image       : String,
}

impl Character {

    pub fn new(name: String, path: Path, image: String, requirements: Vec<String>, directions: Vec<String>) -> Character {
        let recruitment_info = RecruitmentInfo {
            requirements,
            directions,
        };

        Character {
            name,
            path,
            recruitment: recruitment_info,
            image
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    // pub fn set_name(&mut self, new_name: String) {
    //     self.name = new_name;
    // }

    pub fn get_path(&self) -> &Path {
        &self.path
    }

    // pub fn set_path(&mut self, new_path: Path) {
    //     self.path = new_path
    // }

    pub fn get_recruitment(&self) -> &RecruitmentInfo {
        &self.recruitment
    }

    // pub fn set_recruitment(&mut self, new_recruitment: RecruitmentInfo) {
    //     self.recruitment = new_recruitment
    // }

    pub fn get_image(&self) -> &String {
        &self.image
    }


}