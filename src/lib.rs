mod parser;
use parser::{Parser, StorageType, DEFAULT_SECTION, SectionItem};
use std::collections::HashMap;


pub struct Neoconf {
    file_path: String,
    storage: StorageType,
}

impl Neoconf {
    pub fn new(file_path: String) -> Self {
        Self { file_path, storage: HashMap::new() }
    }

    pub fn load(&mut self) {
        let file_contents = self.get_file_contents();
        let mut parser = Parser::new(self.file_path.to_string(), file_contents);

        self.storage = parser.parse();
    }

    pub fn save(&self) {}

    pub fn get(&self, section: Option<&str>, key: &str) -> Option<String> {
        let section_name = get_section_name(section);

        match self.storage.get(&section_name) {
            Some(section_items) => {
                for item in section_items.iter() {
                    if item.key == key {
                        return Some(item.value.to_owned())
                    }
                }

                Some(String::new())
            },
            None => None,
        }
    }

    pub fn set(&mut self, section: Option<&str>, key: &str, value: &str) {
        let section_name = get_section_name(section);

        let item = SectionItem {
            key: key.to_string(),
            value: value.to_string()
        };

        match self.storage.get_mut(&section_name) {
            Some(section_items) => {
                section_items.push(item)
            }
            None => {
                self.storage.insert(section_name.to_owned(), vec![item]);
            }
        }
    }

    pub fn remove(&mut self, section: Option<&str>, key: &str) -> bool {
        let section_name = get_section_name(section);

        // self.storage.remove(&key);

        match self.storage.get_mut(&section_name) {
            Some(section_items) => {
                for (index, item) in section_items.iter().enumerate() {
                    if item.key == key {
                        section_items.remove(index);
                        return true;
                    }
                }

                return false;
            },
            None => false,
        }
    }

    /// read config file and return file contents
    fn get_file_contents(&self) -> String {
        if std::path::Path::new(&self.file_path).exists() == false {
            std::fs::write(&self.file_path, "").expect("Failed to write file");
        }

        let contents = std::fs::read_to_string(&self.file_path).expect("Failed to read file");

        return contents;
    }
}

fn get_section_name(section: Option<&str>) -> String {
    match section {
        Some(x) => {
            return x.to_string()
        },
        None => {
            return DEFAULT_SECTION.to_string()
        }
    }
}

#[cfg(test)]
mod test;
