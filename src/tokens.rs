#[derive(Debug, PartialEq)]
pub enum Token {
    Comment(String),
    Section(String),
    LParen,
    RParen,
    Key(String),
    Equals,
    Value(String),
}
