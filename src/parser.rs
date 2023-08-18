use std::slice::Iter;

use crate::tokens::Token;

pub struct Section {
    pub name: String,
    pub comments: Vec<String>,
    pub kv: Vec<(String, String)>,
    pub children: Vec<Section>,
}

pub struct Parser {
    tokens: Vec<Token>,
    read_pos: usize,
    pub root_section: Section,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            read_pos: 0,
            root_section: Section {
                name: String::from("root"),
                comments: Vec::new(),
                kv: Vec::new(),
                children: Vec::new(),
            },
        }
    }

    pub fn parse(&mut self) {}
}
