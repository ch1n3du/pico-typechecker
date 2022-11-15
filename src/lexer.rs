use std::ops::Range;

use crate::token::Token;

use chumsky::prelude::*;

pub type Span = Range<usize>;
pub type Spanned<T> = (T, Span);

pub fn lexer() -> impl Parser<char, Vec<Spanned<Token>>, Error = Simple<char>> {
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
        just("!=").to(Token::NotEqual),
        just('!').to(Token::Not),
        just("==").to(Token::EqualEqual),
        just('=').to(Token::Equal),
        just("<=").to(Token::LessEqual),
        just('<').to(Token::Less),
        just(">=").to(Token::GreaterEqual),
        just('>').to(Token::Greater),
    ));

    let grouping = choice((
        // just("()").to(Token::Unit),
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
        "fn" => Token::Fn,
        "if" => Token::If,
        "else" => Token::Else,
        "let" => Token::Let,
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
