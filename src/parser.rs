use crate::tokens::Token;

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub comments: Vec<String>,
    pub kv: Vec<(String, String)>,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(name: &String) -> Self {
        Self {
            name: name.to_string(),
            comments: Vec::new(),
            kv: Vec::new(),
            children: Vec::new(),
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    read_pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            read_pos: 0,
        }
    }

    fn peek(&self) -> Option<&Token> {
        if self.read_pos == self.tokens.len() - 1 {
            return None;
        }

        Some(&self.tokens[self.read_pos + 1])
    }

    fn get_token(&self) -> Option<&Token> {
        if self.read_pos == self.tokens.len() {
            return None;
        }

        Some(&self.tokens[self.read_pos])
    }

    pub fn parse(&mut self) -> Node {
        let mut root_node = Node::new(&"root".to_string());

        while let Some(token) = self.get_token() {
            println!("Token: {:?}", token);

            self.read_pos += 1;
        }

        root_node
    }
}
