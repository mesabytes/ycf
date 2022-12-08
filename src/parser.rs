use std::{collections::HashMap, process::exit};

#[derive(PartialEq)]
pub struct SectionItem {
    pub key: String,
    pub value: String,
}

// type for parsed data in hashmap
pub type StorageType = HashMap<String, Vec<SectionItem>>;
pub const DEFAULT_SECTION: &str = "main";
const COMMENT_CHAR: &str = ";";
const DELIMITER: &str = "=";

pub struct Parser {
    file_path: String,
    file_contents: String,
    current_section: String,
}

impl Parser {
    pub fn new(file_path: String, file_contents: String) -> Self {
        Self {
            file_path,
            file_contents,
            current_section: DEFAULT_SECTION.to_string(),
        }
    }

    pub fn parse(&mut self) -> StorageType {
        // temp storage for parsed data
        let mut storage: StorageType = HashMap::new();
        let mut inside_section = false;

        for (index, line) in self.file_contents.lines().enumerate() {
            let line = line.trim();
            let line_number = index + 1;

            if line.starts_with(COMMENT_CHAR) || line.is_empty() {
                continue;
            }

            if line.starts_with("section") && line.ends_with("{") {
                self.current_section = get_new_section(line);

                inside_section = true;
            }

            if inside_section == true && line.starts_with("}") || line.ends_with("}") {
                self.current_section = DEFAULT_SECTION.to_string();

                inside_section = false;
            }

            if line.contains(DELIMITER) {
                let (mut key, mut value) = line.split_once("=").expect("Corrupt config file!");

                key = key.trim();
                value = value.trim();

                if key.is_empty() && value.is_empty() {
                    println!("[neoconf] ParserError: `no key or value found`, line {line_number} in '{}'", self.file_path);
                    exit(1);
                }

                if key.is_empty() {
                    println!(
                        "[neoconf] Error: `no key found for value`, line {line_number} in '{}'",
                        self.file_path
                    );
                    exit(1);
                }

                if value.is_empty() {
                    println!("[neoconf] Error: `no value found for key '{key}'`, line {line_number} in '{}'", self.file_path);
                    exit(1);
                }

                let item = SectionItem {
                    key: key.to_string(),
                    value: value.to_string(),
                };

                match storage.get_mut(&self.current_section) {
                    Some(section_items) => section_items.push(item),
                    None => {
                        storage.insert(self.current_section.to_owned(), vec![item]);
                    }
                }
            }
        }

        storage
    }
}

fn get_new_section(line: &str) -> String {
    let new_line: Vec<&str> = line.split(" ").collect();

    // index 0: "section"
    // index 1: section name
    // index 2: {
    let new_section = new_line[1];

    if new_section.is_empty() {
        return DEFAULT_SECTION.to_string();
    }

    return new_section.to_string();
}
