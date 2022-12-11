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
    file: String,
    storage: BTreeMap<String, String>
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
            file: file.to_string(),
            storage
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        return self.storage.get(key).cloned()
    }

    pub fn set(&mut self, key: String, value: String) -> Option<String> {
        self.storage.insert(key, value)
    }
}

#[cfg(test)]
mod test;
