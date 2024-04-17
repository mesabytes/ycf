use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::tokens::Token;

const INPUT: &str = r#"
# Section comment
@main {
    # key comment
    username = mark;
    password = pass;

    @host_info {
        host = localhost
    }
}
"#;

#[test]
fn lexer() {
    let mut lexer = Lexer::new(INPUT);

    let tokens = lexer.tokenize();

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
            Token::Section("host_info".into()),
            Token::LParen,
            Token::Key("host".into()),
            Token::Equals,
            Token::Value("localhost".into()),
            Token::RParen,
            Token::RParen,
        ]
    );
}

#[test]
fn parser() {
    let mut lexer = Lexer::new(INPUT);

    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);

    let tree = parser.parse();

    println!("tree: {:#?}", tree);

    assert_eq!(true, true)
}
