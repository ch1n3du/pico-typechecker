use std::vec;

use chumsky::Parser;

use pico_typechecker::{
    ast::Expr,
    lexer::{lexer, Span},
    parser,
    tipo::Tipo,
    token::Token,
    typechecker::*,
};

fn try_parsing(src: &str) -> Expr {
    let toks: Vec<(Token, Span)> = lexer().parse(src).unwrap();
    parser::expr_parser()
        .parse(chumsky::Stream::from_iter(1..1, toks.into_iter()))
        .unwrap()
}

#[test]
fn dummy() {
    let expr = try_parsing(
        "funk fib(n: int) -> int { 
                if n < 2 {
                    n
                } else {
                    fib(n-1) + fib(n-2)
                }
            }
    ",
    );
    let mut checker = TypeChecker::new();

    let tipo = checker
        .check_expr(&expr)
        .unwrap_or_else(|e| panic!("Type Error: {e}"));

    // panic!("Tipo: '{tipo}'");

    assert_eq!(
        tipo,
        Tipo::Fn {
            args: vec![Tipo::int_type()],
            ret: Box::new(Tipo::int_type())
        }
    )
}
