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

    pub fn set(&self, section: &str, key: &str, value: &str) {
        todo!();
    }

    pub fn remove(&mut self, section: Option<&str>, key: &str) {
        // let section = match section {
        //     Some(value) => value,
        //     None => {
        //         DEFAULT_SECTION
        //     }
        // };


        // let mut file_contents = self.get_file_contents();
        // let file_contents_clone = &file_contents.clone();

        // let mut current_section = DEFAULT_SECTION;
        // let mut inside_section = false;

        // for (index, line) in file_contents_clone.lines().enumerate() {
        //     if skip_comments_and_empty_lines(line) {
        //         continue;
        //     }

        //     if line.starts_with("section") && line.ends_with("{") {
        //         current_section = get_new_section(line);

        //         inside_section = true;
        //     }

        //     if inside_section == true && line.starts_with("}") {
        //         inside_section = false;
        //         current_section = DEFAULT_SECTION;
        //     }

        //     if line.contains("=") {
        //         let (mut pair_key, _) = line.split_once("=").expect("Corrupt config file!");
                
        //         pair_key = pair_key.trim();
                
        //         if current_section == section && pair_key == key {
        //             let mut new_contents: Vec<&str> = file_contents.split("\n").collect();

        //             new_contents.remove(index);

        //             file_contents = new_contents.join("\n");

        //         }
        //     }
        // }

        // self.storage.remove(&format!("{}.{}", section, key));
        // std::fs::write(&self.file_path, file_contents).expect("Failed to write file");
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
    
    // fn parse(&mut self, file_contents: String) {
    //     let mut current_section = DEFAULT_SECTION;
    //     let mut inside_section = false;

    //     for line in file_contents.lines() {
    //         if skip_comments_and_empty_lines(line) {
    //             continue;
    //         }

    //         // section NAME {
    //         // }
    //         if line.starts_with("section") && line.ends_with("{") {
    //             current_section = get_new_section(line);

    //             inside_section = true;
    //         }

    //         if inside_section == true && line.starts_with("}") {
    //             inside_section = false;
    //             current_section = DEFAULT_SECTION;
    //         }

    //         if line.contains("=") {
    //             let (mut key, mut value) = line.split_once("=").expect("Corrupt config file!");
                
    //             key = key.trim();
    //             value = value.trim();
                
    //             self.hash_map.insert(format!("{}.{}", current_section, key), value.to_owned());
    //         }

    //     }
    // }

}

// fn remove_first_and_last_chars(value: &str) -> &str {
//     let mut chars = value.chars();
//     chars.next();
//     chars.next_back();
//     chars.as_str()
// }

#[cfg(test)]
mod test;
