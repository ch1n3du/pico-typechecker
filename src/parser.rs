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
    ast::{Expr, Op},
    function::Function,
    lexer::Span,
    tipo::Tipo,
    token::Token,
    value::Value,
};

pub fn expr_parser() -> impl Parser<Token, Expr, Error = Simple<Token>> {
    recursive(|raw_expr| {
        let value = select! {
            Token::Unit => Value::Unit,
            Token::Number { value } => Value::Int(value.parse().unwrap()),
            Token::String { value } => Value::Str(Box::new(value)),
            Token::Bool { value } => Value::Bool(value.parse().unwrap()),
        }
        .map_with_span(|value: Value, location: Span| Expr::Value { value, location });

        let raw_ident = select! {Token::Identifier { value } => value.clone()};
        let ident = raw_ident
            .map_with_span(|value: String, location: Span| Expr::Identifier { value, location })
            .labelled("Identifier");

        let grouping = raw_expr
            .clone()
            .delimited_by(just(Token::LeftParen), just(Token::RightParen));

        let primary = choice((value, ident, grouping));

        let args = raw_expr
            .clone()
            .separated_by(just(Token::Comma))
            .then_ignore(just(Token::Comma).or_not())
            .delimited_by(just(Token::LeftParen), just(Token::RightParen));

        let call = primary
            .clone()
            .then(args.repeated().or_not())
            .map_with_span(|(initial_callee, args), location: Span| {
                if let Some(args) = args {
                    args.into_iter()
                        .fold(initial_callee, |callee, args_| Expr::Call {
                            callee: Box::new(callee),
                            args: args_,
                            location: location.clone(),
                        })
                } else {
                    initial_callee
                }
            });

        let unary_op = choice((
            just(Token::Not).to(Op::Not),
            just(Token::Minus).to(Op::Minus),
        ));

        let unary = unary_op
            .repeated()
            .then(call)
            .map_with_span(|(ops, expr), location: Span| {
                ops.into_iter().rev().fold(expr, |acc, op| Expr::Unary {
                    op,
                    rhs: Box::new(acc),
                    location: location.clone(),
                })
            });
        // .foldr(|op, rhs| Expr::Unary {
        // op,
        // rhs: Box::new(rhs),
        // });

        // factor ::= unary (( * | / ) factor)* ;
        let factor_op = just(Token::RSlash)
            .to(Op::Divide)
            .or(just(Token::Star).to(Op::Multiply));

        let factor = unary
            .clone()
            .then(factor_op.then(unary).repeated())
            .map_with_span(|(lhs, rhss), location: Span| {
                rhss.into_iter().fold(lhs, |acc, (op, rhs)| Expr::Binary {
                    lhs: Box::new(acc),
                    op,
                    rhs: Box::new(rhs),
                    location: location.clone(),
                })
            });

        // term ::= factor (( + | - )  factor)* ;
        let term_op = just(Token::Plus)
            .to(Op::Plus)
            .or(just(Token::Minus).to(Op::Minus));

        let term = factor
            .clone()
            .then(term_op.then(factor).repeated())
            .map_with_span(|(lhs, rhss), location: Span| {
                rhss.into_iter().fold(lhs, |acc, (op, rhs)| Expr::Binary {
                    lhs: Box::new(acc),
                    op,
                    rhs: Box::new(rhs),
                    location: location.clone(),
                })
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
            .map_with_span(|(lhs, rhss), location: Span| {
                rhss.into_iter().fold(lhs, |acc, (op, rhs)| Expr::Binary {
                    lhs: Box::new(acc),
                    op,
                    rhs: Box::new(rhs),
                    location: location.clone(),
                })
            });

        // equality ::= comparison (( == | != ) comparison)* ;
        let equality_op = just(Token::EqualEqual)
            .to(Op::EqualEqual)
            .or(just(Token::EqualEqual).to(Op::EqualEqual));

        let equality = comparison
            .clone()
            .then(equality_op.then(comparison).repeated())
            .map_with_span(|(lhs, rhss), location: Span| {
                rhss.into_iter().fold(lhs, |acc, (op, rhs)| Expr::Binary {
                    lhs: Box::new(acc),
                    op,
                    rhs: Box::new(rhs),
                    location: location.clone(),
                })
            })
            .labelled("equality");

        // logicalAnd ::= equality (( and | or ) equality)* ;
        let logical_and = equality
            .clone()
            .then(just(Token::And).to(Op::And).then(equality).repeated())
            .map_with_span(|(lhs, rhss), location: Span| {
                rhss.into_iter().fold(lhs, |acc, (op, rhs)| Expr::Binary {
                    lhs: Box::new(acc),
                    op,
                    rhs: Box::new(rhs),
                    location: location.clone(),
                })
            })
            .labelled("logical_and");

        // logicalOr ::= logicalAnd (or logicalAnd)* ;
        let logical_or = logical_and
            .clone()
            .then(just(Token::Or).to(Op::Or).then(logical_and).repeated())
            .map_with_span(|(lhs, rhss), location: Span| {
                rhss.into_iter().fold(lhs, |acc, (op, rhs)| Expr::Binary {
                    lhs: Box::new(acc),
                    op,
                    rhs: Box::new(rhs),
                    location: location.clone(),
                })
            })
            .labelled("logical_or");

        let block = raw_expr
            .clone()
            .delimited_by(just(Token::LeftBrace), just(Token::RightBrace))
            .map_with_span(|e, location| Expr::Block {
                expr: Box::new(e),
                location,
            })
            .labelled("block");

        let else_block = just(Token::Else)
            .ignore_then(block.clone())
            .or_not()
            .map_with_span(|maybe_block, location| {
                if let Some(block) = maybe_block {
                    block
                } else {
                    Expr::Value {
                        value: Value::Unit,
                        location,
                    }
                }
            })
            .labelled("Else Block");

        let if_ = just(Token::If)
            .ignore_then(logical_or.clone())
            .then(block.clone())
            .then(else_block)
            .map_with_span(
                |((condition, truthy_branch), falsy_branch), location| Expr::If {
                    condition: Box::new(condition),
                    truthy_branch: Box::new(truthy_branch),
                    falsy_branch: Box::new(falsy_branch),
                    location,
                },
            )
            .labelled("If Expression");

        let tipo = recursive(|raw_tipo| {
            just(Token::Fn)
                .ignore_then(
                    raw_tipo
                        .clone()
                        .separated_by(just(Token::Comma))
                        .then_ignore(just(Token::Comma).or_not())
                        .delimited_by(just(Token::LeftParen), just(Token::RightParen))
                        .then_ignore(just(Token::RArrow))
                        .then(raw_tipo.clone())
                        .map(|(args, ret): (Vec<Tipo>, Tipo)| Tipo::new_fn(args, ret)),
                )
                .or(raw_ident.map(|name| Tipo::new(name.as_str())))
        });

        // annotation ::= ':' IDENT
        let annotation = just(Token::Colon)
            .ignore_then(tipo.clone())
            .labelled("Type Annotation");

        let then_expr = raw_expr.clone().or_not().map(|e| {
            e.clone().unwrap_or(Expr::Value {
                value: Value::Unit,
                location: 0..0,
            })
        });

        // letExpr ::= 'let' IDENT '=' Expr ; Expr
        let let_ = just(Token::Let)
            .ignore_then(raw_ident)
            .then(annotation.clone().or_not())
            .then_ignore(just(Token::Equal))
            .then(raw_expr.clone())
            .then_ignore(just(Token::SemiColon))
            .then(then_expr.clone())
            .map_with_span(
                |(((name, let_tipo), initializer), then), location| Expr::Let {
                    name,
                    let_tipo,
                    initializer: Box::new(initializer),
                    then: Box::new(then),
                    location,
                },
            )
            .labelled("Let Expression");

        // params ::= ( ((ident annotation) (',' ident annotation)* ','?)?   )
        let params = raw_ident
            .clone()
            .then(annotation.clone())
            .separated_by(just(Token::Comma))
            .then_ignore(just(Token::Comma).or_not())
            .delimited_by(just(Token::LeftParen), just(Token::RightParen))
            .labelled("Function Parameters");

        // funkAnnotation ::= '->' IDENT
        let return_annotation = just(Token::RArrow)
            .ignore_then(tipo.clone())
            .or_not()
            .labelled("Funk Type Annotation");

        // funkDecl ::= funk IDENT params block
        let funk = params
            .then(return_annotation)
            .then(block.clone())
            .map(|((params, ret), body)| {
                let ret = ret.unwrap_or(Tipo::unit_type());

                Function {
                    params,
                    ret,
                    body: Box::new(body),
                }
            })
            .labelled("Function body");
        let funk_decl = just(Token::Funk)
            .ignore_then(raw_ident.clone())
            .then(funk.clone())
            .then(then_expr.clone())
            .map_with_span(
                |((name, funk), then): ((String, Function), Expr), location| Expr::Funk {
                    name,
                    fn_: funk,
                    location,
                    then: Box::new(then),
                },
            );

        let fn_ = just(Token::Fn)
            .ignore_then(funk)
            .map_with_span(|fn_, location| Expr::Value {
                value: Value::Fn(Box::new(fn_)),
                location,
            });

        choice((block, let_, logical_or, if_, funk_decl, fn_))
    })
}
