use crate::tokens::Token;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Default)]
struct State {
    reading_value: bool,
}

fn is_allowd_word(ch: char) -> bool {
    ch.is_ascii_lowercase() || ch.is_uppercase() || ch.is_ascii_digit() || ch == '_'
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    state: State,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
            state: State::default(),
        }
    }

    fn get_token(&mut self, ch: char) -> Option<Token> {
        match ch {
            '=' => {
                self.state.reading_value = true;
                Some(Token::Equals)
            }
            ';' | '\n' => {
                self.state.reading_value = false;
                None
            }
            '{' => Some(Token::LParen),
            '}' => Some(Token::RParen),
            '@' => {
                let mut section_name = String::new();

                while let Some(peek) = self.input.peek() {
                    if !is_allowd_word(*peek) {
                        break;
                    }

                    section_name.push(*peek);
                    self.input.next();
                }

                Some(Token::Section(section_name))
            }
            '#' => {
                let mut comment = String::new();

                while let Some(peek) = self.input.peek() {
                    if *peek == '\n' {
                        break;
                    }

                    comment.push(*peek);
                    self.input.next();
                }

                Some(Token::Comment(comment))
            }
            'a'..='z' | 'A'..='Z' => {
                let mut ident = String::from(ch);

                while let Some(peek) = self.input.peek() {
                    if !is_allowd_word(*peek) {
                        break;
                    }
                    ident.push(*peek);
                    self.input.next();
                }

                if self.state.reading_value {
                    Some(Token::Value(ident))
                } else {
                    Some(Token::Key(ident))
                }
            }
            _ => None,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.input.next() {
            if let Some(token) = self.get_token(ch) {
                tokens.push(token)
            }
        }

        tokens
    }
}
