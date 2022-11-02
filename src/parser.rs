use std::os::raw;

/// This module contains a parser for the language
/// Language Specification in  Pseudo EBNF
/// TODO Add If/Else
///
/// rawExpr ::=
///  
/// logicalOr ::= logicalAnd (or logicalAnd)* ;
///
/// logicalAnd ::= equality (( and | or ) equality)* ;
///
/// equality ::= comparison (( == | != ) comparison)* ;
///
/// comparison ::= term (( < | <= | > | >= ) term)* ;
///
/// term ::= factor (( + | - )  factor)* ;
///
/// factor ::= unary (( * | / ) factor)* ;
///
/// unary ::= (- | not) unary | primary ;
///
/// primary ::= IDENTIFIER | NUMBER | STRING | BOOL | UNIT ;

/// Import Chumsky and get to work
use chumsky::prelude::*;

use crate::{
    ast::{Expr, Op, Tipo, Value},
    token::Token,
};

fn expr_parser() -> impl Parser<Token, Expr, Error = Simple<Token>> {
    recursive(|raw_expr| {
        // primary ::= IDENTIFIER | NUMBER | STRING | BOOL | UNIT ;
        let primary = select! {
            Token::Unit => Expr::Value(Value::Unit),
            Token::Number { value } => Expr::Value(Value::Num(value.parse().unwrap())),
            Token::String { value } => Expr::Value(Value::Str(value)),
            Token::Bool { value } => Expr::Value(Value::Bool(value.parse().unwrap())),
            Token::Identifier { value } => Expr::Identifier(value)
        };

        // unary ::= (- | not) unary | primary ;
        let unary_op = choice((
            just(Token::Not).to(Op::Not),
            just(Token::Minus).to(Op::Minus),
        ));

        let unary = unary_op
            .repeated()
            .then(primary)
            .foldr(|op, rhs| Expr::Unary {
                op,
                rhs: Box::new(rhs),
            });

        // factor ::= unary (( * | / ) factor)* ;
        let factor_op = just(Token::RSlash)
            .to(Op::Divide)
            .or(just(Token::Star).to(Op::Multiply));

        let factor =
            unary
                .clone()
                .then(factor_op.then(unary).repeated())
                .foldl(|lhs, (op, rhs)| Expr::Binary {
                    lhs: Box::new(lhs),
                    op,
                    rhs: Box::new(rhs),
                });

        // term ::= factor (( + | - )  factor)* ;
        let term_op = just(Token::Plus)
            .to(Op::Plus)
            .or(just(Token::Minus).to(Op::Minus));

        let term = factor
            .clone()
            .then(term_op.then(factor).repeated())
            .foldl(|lhs, (op, rhs)| Expr::Binary {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            });

        // comparison ::= term (( < | <= | > | >= ) term)* ;
        let comparison_op = choice((
            just(Token::Less).to(Op::Less),
            just(Token::LessEqual).to(Op::LessEqual),
            just(Token::Greater).to(Op::GreaterEqual),
        ));

        let comparison = term
            .clone()
            .then(comparison_op.then(term).repeated())
            .foldl(|lhs, (op, rhs)| Expr::Binary {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            });

        // equality ::= comparison (( == | != ) comparison)* ;
        let equality_op = just(Token::EqualEqual)
            .to(Op::EqualEqual)
            .or(just(Token::EqualEqual).to(Op::EqualEqual));

        let equality = comparison
            .clone()
            .then(equality_op.then(comparison).repeated())
            .foldl(|lhs, (op, rhs)| Expr::Binary {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            });

        // logicalAnd ::= equality (( and | or ) equality)* ;
        let logical_and = equality
            .clone()
            .then(just(Token::And).to(Op::And).then(equality).repeated())
            .foldl(|lhs, (op, rhs)| Expr::Binary {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            });

        // logicalOr ::= logicalAnd (or logicalAnd)* ;
        let logical_or = logical_and
            .clone()
            .then(just(Token::Or).to(Op::Or).then(logical_and).repeated())
            .foldl(|lhs, (op, rhs)| Expr::Binary {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            });

        logical_or
    })
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use crate::{lexer::lexer, token::Token};
    #[test]
    fn basic() {
        let toks: Vec<Token> = lexer()
            .parse("2 * 3 == 4 / 3;")
            .unwrap()
            .into_iter()
            .map(|p| p.0)
            .collect();
        let exprs = super::expr_parser().parse(toks);
        panic!("{exprs:?}")
    }
}
