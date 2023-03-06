use chumsky::Parser;
use pico_typechecker::{lexer::*, token::Token};

#[test]
fn basic() {
    let tokens = lexer().parse(". + - / * , : ; = == < <= > >= ( ) { } -> ! and or funk let if else this_is_an_identifier 1234 true false \"this is a string\" fn").unwrap();
    let expected = vec![
        Token::Dot,
        Token::Plus,
        Token::Minus,
        Token::RSlash,
        Token::Star,
        Token::Comma,
        Token::Colon,
        Token::SemiColon,
        Token::Equal,
        Token::EqualEqual,
        Token::Less,
        Token::LessEqual,
        Token::Greater,
        Token::GreaterEqual,
        Token::LeftParen,
        Token::RightParen,
        Token::LeftBrace,
        Token::RightBrace,
        Token::RArrow,
        Token::Not,
        Token::And,
        Token::Or,
        Token::Funk,
        Token::Let,
        Token::If,
        Token::Else,
        Token::Identifier {
            value: "this_is_an_identifier".to_string(),
        },
        Token::Int {
            value: "1234".to_string(),
        },
        Token::Bool {
            value: "true".to_string(),
        },
        Token::Bool {
            value: "false".to_string(),
        },
        Token::Str {
            value: "this is a string".to_string(),
        },
        Token::Fn,
    ];

    for (i, tok) in tokens.iter().enumerate() {
        let expected_tok = &expected[i];
        let tok_got = &tok.0;
        assert_eq!(
            tok_got, expected_tok,
            "[{i}]: Expected: {expected_tok:?} but got {tok_got:?}"
        )
    }
}
