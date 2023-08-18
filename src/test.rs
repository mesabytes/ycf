use crate::lexer::{Lexer, Token};

const INPUT: &str = r#"
# Section comment
@main {
    # key comment
    username = mark;
    password = pass;
}
"#;

#[test]
fn tokenize() {
    let mut lexer = Lexer::new(INPUT);

    let tokens = lexer.tokenize();

    println!("Tokens: {:?}", tokens);

    assert_eq!(
        tokens,
        vec![
            Token::Comment(" Section comment".into()),
            Token::Section("main".into()),
            Token::LParen,
            Token::Comment(" key comment".into()),
            Token::Key("username".into()),
            Token::Equals,
            Token::Value("mark".into()),
            Token::Key("password".into()),
            Token::Equals,
            Token::Value("pass".into()),
            Token::RParen,
        ]
    );
}
