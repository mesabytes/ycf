mod parser;
use parser::parse;
use std::collections::BTreeMap;

pub struct Ycf {
    storage: BTreeMap<String, String>,
    default_storage: BTreeMap<String, String>,
    file: Option<String>,
    auto_save: bool,
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
            auto_save: false,
        }
    }

    pub fn load_from_string(input_string: String) -> Self {
        let mut storage = BTreeMap::new();

        parse(input_string, &mut storage);

        Self {
            storage,
            default_storage: BTreeMap::new(),
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
        parse(input_string, &mut self.default_storage);
    }

    // SETTINGS --------

    pub fn get(&self, key: &str) -> Option<String> {
        match self.storage.get(key).cloned() {
            Some(value) => Some(value),
            None => self.default_storage.get(key).cloned(),
        }
    }

    pub fn set(&mut self, key: String, value: String) -> Option<String> {
        let item = self.storage.insert(key, value);

        if self.auto_save {
            self.save(None);
        }

        item
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
            Some(file) => {}
            None => {
                return;
            }
        }
    }
}

#[cfg(test)]
mod test;
