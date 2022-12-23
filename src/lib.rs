mod parser;
mod constants;
mod preprocessor;

use constants::ROOT_SECTION;
use parser::{parse, Node};

pub struct Ycf {
    root_node: Node,
    default_root_node: Node,
    file: Option<String>,
    auto_save: bool,
}

impl Ycf {
    pub fn load_from_file(file: &str) -> Self {
        if std::path::Path::new(&file).exists() == false {
            std::fs::write(&file, "").expect("Failed to write file");
        }

        let file_content = std::fs::read_to_string(&file).expect("Failed to read file");

        Self {
            root_node: parse(Some(file.to_string()), file_content),
            default_root_node: Node::new(ROOT_SECTION.into()),
            file: Some(file.into()),
            auto_save: false,
        }
    }

    pub fn load_from_string(input_string: String) -> Self {
        Self {
            root_node: parse(None, input_string),
            default_root_node: Node::new(ROOT_SECTION.into()),
            file: None,
            auto_save: false,
        }
    }

    // -------- SETTINGS

    /// Turn on/off auto save
    pub fn auto_save(&mut self, b: bool) {
        self.auto_save = b;
    }

    pub fn default_config(&mut self, input_string: String) {
        self.default_root_node = parse(self.file.clone(), input_string);
    }

    /// reparse config file
    /// **Note: only works when loading from a file**
    pub fn reload(&mut self) {
        if let Some(f) = self.file.clone() {
            let file_content = std::fs::read_to_string(&f).expect("Failed to read file");

            self.root_node = parse(Some(f), file_content);
        }
    }

    // SETTINGS --------

    pub fn get(&self, key: &str) -> Option<String> {
        unimplemented!()
    }

    pub fn set(&mut self, key: String, value: String) -> Option<String> {
        unimplemented!()
    }

    pub fn remove(&mut self, key: String) {
        unimplemented!()
    }

    /// save
    pub fn save(&self, file: Option<String>) {
        let save_file: Option<String> = {
            match file {
                Some(f) => Some(f),
                None => self.file.clone(),
            }
        };

        match save_file {
            Some(file) => {
                let text = self.root_node.convert_to_string();

                std::fs::write(&file, text).expect("Failed to save config");
            }
            None => {
                return;
            }
        }
    }
}

#[cfg(test)]
mod test;
