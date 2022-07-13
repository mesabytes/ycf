use std::collections::HashMap;

// type for parsed data in hashmap
pub type StorageType = HashMap<String, String>;

pub struct Parser {
    file_contents: String,
}

/*
[1] get file content (as String)
[2] parse string and extract keywords and names and keys and values
[3] save section in hashmap named data
[?] add method to write data back to config file as string
[!] Note: comments will be overwritten
*/

impl Parser {
    pub fn new(file_contents: String) -> Self {
        Self {
            file_contents
        }
    }

    pub fn parse(&self) -> StorageType {
        // temp storage for parsed data
        let storage: StorageType = HashMap::new();

        

        storage
    }
}