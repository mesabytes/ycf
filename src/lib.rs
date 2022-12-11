mod parser;
use std::collections::BTreeMap;

use parser::parse;

// pub struct Options {
//     pub auto_save: bool,

//     /// Parse config from a string not from a file (used for testing or something? idk)
//     pub from_string: Option<String>,

//     /// `Default configuration`
//     /// used when config does not exists or no from string is provided
//     pub default: Option<String>,
// }

// impl Default for Options {
//     fn default() -> Self {
//         Self {
//             auto_save: false,
//             from_string: None,
//             default: None,
//         }
//     }
// }

pub struct Ycf {
    storage: BTreeMap<String, String>,
    default_storage: BTreeMap<String, String>,
    file: Option<String>,
    auto_save: bool
}

impl Ycf {
    pub fn load_from_file(file: &str) -> Self {
        if std::path::Path::new(&file).exists() == false {
            std::fs::write(&file, "").expect("Failed to write file");
        }

        let file_content = std::fs::read_to_string(&file).expect("Failed to read file");
        let mut storage = BTreeMap::new();

        parse(file_content, &mut storage);

        Self {
            storage,
            default_storage: BTreeMap::new(),
            file: Some(file.into()),
            auto_save: false
        }
    }

    pub fn load_from_string(input_string: String) -> Self {
        let mut storage = BTreeMap::new();

        parse(input_string, &mut storage);

        Self {
            storage,
            default_storage: BTreeMap::new(),
            file: None,
            auto_save: false
        }
    }

    // -------- SETTINGS

    /// Turn on/off auto save
    pub fn auto_save(&mut self, b: bool) {
        self.auto_save = b;
    }

    pub fn default_config(&mut self, input_string: String) {
        parse(input_string, &mut self.default_storage);
    }

    // SETTINGS --------

    pub fn get(&self, key: &str) -> Option<String> {
        match self.storage.get(key).cloned() {
            Some(value) => Some(value),
            None => {
                self.default_storage.get(key).cloned()
            }
        }
    }

    pub fn set(&mut self, key: String, value: String) -> Option<String> {
        let item = self.storage.insert(key, value);

        if self.auto_save {
            self.save();
        }

        item
    }

    /// save
    pub fn save(&self) {
        match &self.file {
            Some(file) => {
                // TODO: Write self.storage back to disk

                println!("file: {file}");
            }
            None => {
                return;
            }
        }
    }
}

#[cfg(test)]
mod test;
