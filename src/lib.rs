mod parser;
use parser::{Parser, StorageType, DEFAULT_SECTION};
use std::collections::HashMap;


pub struct Neoconf {
    file_path: String,
    storage: StorageType,
}

impl Neoconf {
    pub fn new(file_path: String) -> Self {
        Self { file_path, storage: HashMap::new() }
    }

    fn get_section_name(&self, section: Option<&str>) -> String {
        match section {
            Some(x) => {
                return x.to_string()
            },
            None => {
                return DEFAULT_SECTION.to_string()
            }
        }
    }

    pub fn get(&self, section: Option<&str>, key: &str) -> Option<&String> {
        let section_name = self.get_section_name(section);

        return self.storage.get(&format!("{section_name}.{key}"))
    }

    pub fn set(&mut self, section: Option<&str>, key: &str, value: &str) {
        let section_name = self.get_section_name(section);

        let key = format!("{section_name}.{key}");

        self.storage.insert(key, value.to_string());
    }

    pub fn remove(&mut self, section: Option<&str>, key: &str) {
        let section_name = self.get_section_name(section);

        let key = format!("{section_name}.{key}");

        self.storage.remove(&key);
    }

    pub fn load(&mut self) {
        let file_contents = self.get_file_contents();
        let mut parser = Parser::new(self.file_path.to_string(), file_contents);

        self.storage = parser.parse();
        // self.parse(file_contents);
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

#[cfg(test)]
mod test;
