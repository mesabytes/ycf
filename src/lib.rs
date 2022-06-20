pub mod neoconf {
    use std::collections::HashMap;

    const DEFAULT_SECTION: &str = "main";

    pub struct Neoconf {
        file_path: String,
        hash_map: HashMap<String, String>,
    }

    impl Neoconf {
        pub fn new(file_path: String) -> Self {
            Self { file_path, hash_map: HashMap::new() }
        }

        pub fn get(&self, section: &mut str, key: &str) -> Option<&String> {
        if section.is_empty() {
            self.hash_map.get(&format!("{DEFAULT_SECTION}.{key}"))
        } else {
            self.hash_map.get(&format!("{section}.{key}"))
        }
    }

        pub fn set(&self, section: &str, key: &str, value: &str) {
            todo!();
        }

        pub fn remove(&self, section: &str, key: &str) {
            todo!();
        }

        pub fn load(&self) {
            let file_contents = self.get_file_contents();

            self.parse(file_contents);
        }

        /// read config file and return file contents
        fn get_file_contents(&self) -> String {
            if std::path::Path::new(&self.file_path).exists() == false {
                std::fs::write(&self.file_path, "").expect("Failed to write file");
            }

            let contents = std::fs::read_to_string(&self.file_path).expect("Failed to read file");

            return contents;
        }

        fn parse(&self, file_contents: String) {

            let mut current_section = DEFAULT_SECTION;

            for line in file_contents.lines() {
                if line.starts_with(";") || line.starts_with("#") || line.is_empty() {
                    continue;
                }

                if line.starts_with("[") && line.ends_with("]") {
                    // change current section
                    let new_section = remove_first_and_last_chars(line);
                    
                    if new_section.is_empty() {
                        current_section = DEFAULT_SECTION
                    } else {
                        current_section = new_section;
                    }
                }

                if line.contains("=") {
                    // is setting
                    // split and trim
                }

            }
        }

    }

    fn remove_first_and_last_chars(value: &str) -> &str {
        let mut chars = value.chars();
        chars.next();
        chars.next_back();
        chars.as_str()
    }

}

#[cfg(test)]
mod tests {
    use crate::neoconf;

    #[test]
    fn it_works() {
        let config = neoconf::Neoconf::new(String::from("test.neoconf"));
        
        assert_eq!(2 + 2, 4);
    }
}
