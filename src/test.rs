use crate::lexer::{Lexer, Token};

// const INPUT: &str = r#"
// # @SECTION_NAME
// @main {
//     # KEY = VALUE;
//     username = mark;
//     password = 123456;
// }
// "#;

const INPUT: &str = r#"
# @SECTION_NAME
# KEY = VALUE;
username = mark;
password = pass;
"#;

#[test]
fn tokenize() {
    let mut lexer = Lexer::new(INPUT);

    let tokens = lexer.tokenize();

    assert_eq!(
        tokens,
        vec![
            Token::Comment(" @SECTION_NAME".into()),
            Token::Comment(" KEY = VALUE;".into()),
            Token::Key("username".into()),
            Token::Equals,
            Token::Value("mark".into()),
            Token::Key("password".into()),
            Token::Equals,
            Token::Value("pass".into()),
        ]
    );
}
