mod parser;
use std::collections::HashMap;

pub struct Options {
    pub auto_save: bool,

    /// Parse config from a string not from a file (used for testing or something? idk)
    pub from_string: Option<String>,

    /// `Default configuration`
    /// used when config does not exists or no from string is provided
    pub default: Option<String>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            auto_save: false,
            from_string: None,
            default: None,
        }
    }
}

pub struct Neoconf {
    file: String,
    options: Options,
}

impl Neoconf {
    pub fn load(file: &str, options: Options) -> Self {
        // TODO:
        // 1. read `file`
        // 2. pass `file` contents into `parse` function as `input`
        // use return map to assgin `storage`
        Self {
            file: file.to_string(),
            options,
        }
    }

    pub fn get() {
        unimplemented!()
    }

    pub fn set() {
        unimplemented!()
    }
}

#[cfg(test)]
mod test;
