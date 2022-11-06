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
    ast::{Expr, Function, Op, Value},
    tipo::Tipo,
    token::Token,
};

pub fn expr_parser() -> impl Parser<Token, Expr, Error = Simple<Token>> {
    recursive(|raw_expr| {
        // primary ::= IDENTIFIER | NUMBER | STRING | BOOL | UNIT ;
        let primary = select! {
            Token::Unit => Expr::Value(Value::Unit),
            Token::Number { value } => Expr::Value(Value::Num(value.parse().unwrap())),
            Token::String { value } => Expr::Value(Value::Str(value)),
            Token::Bool { value } => Expr::Value(Value::Bool(value.parse().unwrap())),
            Token::Identifier { value } => Expr::Identifier(value)
        }
        .or(raw_expr
            .clone()
            .delimited_by(just(Token::LeftParen), just(Token::RightParen))
            .map(|e| Expr::Grouping(Box::new(e))));

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
            })
            .labelled("equality");

        // logicalAnd ::= equality (( and | or ) equality)* ;
        let logical_and = equality
            .clone()
            .then(just(Token::And).to(Op::And).then(equality).repeated())
            .foldl(|lhs, (op, rhs)| Expr::Binary {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            })
            .labelled("logical_and");

        // logicalOr ::= logicalAnd (or logicalAnd)* ;
        let logical_or = logical_and
            .clone()
            .then(just(Token::Or).to(Op::Or).then(logical_and).repeated())
            .foldl(|lhs, (op, rhs)| Expr::Binary {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            })
            .labelled("logical_or");

        let block = raw_expr
            .clone()
            .delimited_by(just(Token::LeftBrace), just(Token::RightBrace))
            .map(|e| Expr::Block(Box::new(e)))
            .labelled("block");

        let if_ = just(Token::If)
            .ignore_then(logical_or.clone())
            .then(block.clone())
            .then(just(Token::Else).ignore_then(block.clone()).or_not())
            .map(|((condition, truthy_branch), falsy_branch)| Expr::If {
                condition: Box::new(condition),
                truthy_branch: Box::new(truthy_branch),
                falsy_branch: falsy_branch.map(Box::new),
            })
            .labelled("If Expression");

        let ident = select! {Token::Identifier { value } => value.clone()}.labelled("Identifier");

        // annotation ::= ':' IDENT
        let annotation = just(Token::Colon)
            .ignore_then(ident)
            .map(|name| Tipo::new(&name))
            .labelled("Type Annotation");

        // letExpr ::= 'let' IDENT '=' Expr ; Expr
        let let_ = just(Token::Let)
            .ignore_then(ident)
            .then(annotation.clone().or_not())
            .then_ignore(just(Token::Equal))
            .then(raw_expr.clone())
            .then_ignore(just(Token::SemiColon))
            .then(raw_expr.clone())
            .map(|(((name, tipo), initializer), then)| Expr::Let {
                name,
                tipo,
                initializer: Box::new(initializer),
                then: Box::new(then),
            })
            .labelled("Let Expression");

        // params ::= ( ((ident annotation) (',' ident annotation)* ','?)?   )
        let params = ident
            .then(annotation.clone())
            .separated_by(just(Token::Comma))
            .then_ignore(just(Token::Comma).or_not())
            .delimited_by(just(Token::LeftParen), just(Token::RightParen))
            .labelled("Function Parameters");

        // funkAnnotation ::= '->' IDENT
        let funk_annotation = just(Token::RArrow)
            .ignore_then(ident)
            .map(|name| Tipo::new(&name))
            .or_not()
            .labelled("Funk Type Annotation");

        // funkDecl ::= funk IDENT params block
        let funk = params
            .then(funk_annotation)
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
            .ignore_then(ident.clone())
            .then(funk.clone())
            // .then(raw_expr)
            // .map(|((name, funk), then)| Expr::Funk {
            .map(|(name, funk)| Expr::Funk {
                name,
                fn_: funk,
                // then: Box::new(then),
            });

        let fn_ = just(Token::Fn).ignore_then(funk).map(Expr::Fn);

        choice((block, let_, logical_or, if_, funk_decl, fn_))
    })
}
