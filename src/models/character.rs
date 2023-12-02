enum Path {
    Human,
    Fairy,
    Any,
}

struct RecruimentInfo {
    requirements: Vec<String>,
    directions: Vec<String>,
}

struct Character {
    name        : String,
    path        : Path,
    recruitment : RecruimentInfo,
}

impl Character {

    fn new(name: String, path: Path, requirements: Vec<String>, directions: Vec<String>) -> Character {
        let recruitment_info = RecruimentInfo {
            recruitments,
            directions,
        };

        Character {
            name,
            path,
            recruitment: recruitment_info,
        }
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn set_name(&mut self, new_name: String) {
        &self.name = new_name;
    }

    fn get_path(&self) -> &Path {
        &self.path
    }

    fn set_path(&mut self, new_path: String) {
        &self.path = new_path
    }

    fn get_recruitment(&self) -> &RecruimentInfo {
        &self.recruitment
    }

    fn set_recruitment(&mut self, new_recruitment: RecruimentInfo) {
        &self.recruitment = new_recruitment
    }

}