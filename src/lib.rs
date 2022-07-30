mod parser;
use parser::{Parser, StorageType, DEFAULT_SECTION, SectionItem};
use std::collections::HashMap;

pub struct Options {
    pub auto_save: bool,
    /// Parse config from a string not from a file (used for testing or something? idk)
    pub from_string: Option<String>
}

impl Default for Options {
    fn default() -> Self {
        Self { auto_save: false, from_string: None }
    }
}

pub struct Neoconf {
    // TODO: move `file_path` to `Options` or something
    // because if you provide a `from_string` option you don't need `file_path`
    file_path: String,
    options: Options,
    storage: StorageType,
}

impl Neoconf {
    pub fn new(file_path: &str, options: Options) -> Self {
        Self { 
            file_path: file_path.to_string(),
            options: options,
            storage: HashMap::new() 
        }
    }

    pub fn load(&mut self) {
        let config_string: String = {
            match &self.options.from_string {
                Some(x) => x.to_string(),
                None => self.get_file_contents()
            }
        };

        let mut parser = Parser::new(self.file_path.to_string(), config_string);

        self.storage = parser.parse();
    }

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

    pub fn set(&mut self, section: Option<&str>, key: &str, value: &str) -> std::io::Result<()> {
        let section_name = get_section_name(section);

        let item = SectionItem {
            key: key.to_string(),
            value: value.to_string()
        };

        match self.storage.get_mut(&section_name) {
            Some(section_items) => {
                // NOTE: if item.value changes then there will be a duplicate key!
                
                if !section_items.contains(&item) {
                    section_items.push(item)
                }
            }
            None => {
                self.storage.insert(section_name.to_owned(), vec![item]);
            }
        }

        if self.options.auto_save {
            self.save()?;
        }

        Ok(())
    }

    pub fn remove(&mut self, section: Option<&str>, key: &str) -> std::io::Result<()> {
        let section_name = get_section_name(section);

        match self.storage.get_mut(&section_name) {
            Some(section_items) => {
                match section_items.iter().position(|item| *item.key == key.to_string()) {
                    Some(index) => {
                        section_items.remove(index);
                    },
                    None => {}
                }
            },
            None => {},
        };

        if self.options.auto_save {
            self.save()?;
        }

        Ok(())
    }

    /// read config file and return file contents
    fn get_file_contents(&self) -> String {
        if std::path::Path::new(&self.file_path).exists() == false {
            std::fs::write(&self.file_path, "").expect("Failed to write file");
        }

        let contents = std::fs::read_to_string(&self.file_path).expect("Failed to read file");

        return contents;
    }

    pub fn save(&self) -> std::io::Result<()> {
        println!("save: {:?}", self.storage.keys());

        let mut contents = String::new();

        // write DEFAULT_SECTION before any other section
        match self.storage.get(DEFAULT_SECTION) {
            Some(items) => {
                for item in items.iter() {
                    contents.push_str(&format!("{} = {}\n", item.key, item.value));
                }
            },
            None => {}
        }

        for section in self.storage.keys() {
            if section == DEFAULT_SECTION {
                continue;
            }

            contents.push_str(&format!("\nsection {} {{\n", section));

            match self.storage.get(section) {
                Some(items) => {
                    for item in items.iter() {
                        contents.push_str(&format!("\t{} = {}\n", item.key, item.value));
                    }
                },
                None => {}
            }
            
            contents.push_str(&format!("}}\n"));
        }

        std::fs::write(&self.file_path, contents)?;
        Ok(())
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
