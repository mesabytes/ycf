use crate::lexer::{Lexer, Token};

// const INPUT: &str = r#"
// // @SECTION_NAME
// @main {
//     // KEY = VALUE;
//     username = mark;
//     password = 123456;
// }
// "#;

#[test]
fn tokenize() {
    let mut lexer = Lexer::new("username=mark\npassword=pass");

    let tokens = lexer.tokenize();

    assert_eq!(
        tokens,
        vec![
            Token::Key("username".into()),
            Token::Equals,
            Token::Value("mark".into()),
            Token::Key("password".into()),
            Token::Equals,
            Token::Value("pass".into()),
        ]
    );
}
