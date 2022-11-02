use std::ops::Range;

use crate::token::Token;

use chumsky::prelude::*;

type Span = Range<usize>;
type Spanned<T> = (T, Span);

fn lexer() -> impl Parser<char, Vec<Spanned<Token>>, Error = Simple<char>> {
    let op = choice((
        just('+').to(Token::Plus),
        just("->").to(Token::RArrow),
        just('-').to(Token::Minus),
        just('/').to(Token::RSlash),
        just('*').to(Token::Star),
        just('.').to(Token::Dot),
        just(',').to(Token::Comma),
        just(':').to(Token::Colon),
        just(';').to(Token::SemiColon),
        just("==").to(Token::EqualEqual),
        just('=').to(Token::Equal),
        just("<=").to(Token::LessEqual),
        just('<').to(Token::Less),
        just(">=").to(Token::GreaterEqual),
        just('>').to(Token::Greater),
    ));

    let grouping = choice((
        just('(').to(Token::LeftParen),
        just(')').to(Token::RightParen),
        just('{').to(Token::LeftBrace),
        just('}').to(Token::RightBrace),
    ));

    let number = text::int(10)
        .map(|value: String| Token::Number { value })
        .labelled("number");

    let boolean = just("true")
        .or(just("false"))
        .map(|s: &str| Token::Bool {
            value: s.to_string(),
        })
        .labelled("boolean");

    let escape = just('\\').ignore_then(
        just('\\')
            .or(just('/'))
            .or(just('"'))
            .or(just('b').to('\x08'))
            .or(just('f').to('\x0C'))
            .or(just('n').to('\n'))
            .or(just('r').to('\r'))
            .or(just('t').to('\t')),
    );

    let string = just('"')
        .ignore_then(filter(|c| *c != '\\' && *c != '"').or(escape).repeated())
        .then_ignore(just('"'))
        .collect::<String>()
        .map(|value| Token::String { value })
        .labelled("string");

    let keyword = text::ident().map(|s: String| match s.as_str() {
        "funk" => Token::Funk,
        "if" => Token::If,
        "else" => Token::Else,
        "let" => Token::Let,
        "while" => Token::While,
        "for" => Token::For,
        "not" => Token::Not,
        "and" => Token::And,
        "or" => Token::Or,
        _ => Token::Identifier { value: s },
    });

    choice((number, boolean, string, op, grouping, keyword))
        .map_with_span(|token, span| (token, span))
        .padded()
        .repeated()
        .then_ignore(end())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let tokens = lexer().parse(". + - / * , : ; = == < <= > >= ( ) { } -> not and or funk let if else while for this_is_an_identifier 1234 true false \"this is a string\"").unwrap();
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
            Token::While,
            Token::For,
            Token::Identifier {
                value: "this_is_an_identifier".to_string(),
            },
            Token::Number {
                value: "1234".to_string(),
            },
            Token::Bool {
                value: "true".to_string(),
            },
            Token::Bool {
                value: "false".to_string(),
            },
            Token::String {
                value: "this is a string".to_string(),
            },
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
}
