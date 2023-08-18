#[derive(Debug, PartialEq)]
pub enum Token {
    Section(String),
    Identifier(String),
    Key(String),
    Value(String),
    Comment(String),
    LParen,
    RParen,
    Equals,
}
