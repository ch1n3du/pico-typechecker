/// This module contains a parser for the language
/// Language Specification in  Pseudo EBNF
///
/// stmtExpr ::= letStmt | block | ifStmt | whileStmt ;
///
/// logicalOr ::= logicalAnd (or logicalAnd)* ;
///
/// logicalAnd ::= equality (( and | or ) equality)* ;
///
/// ifElse ::= if expression block (else block)*;
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

use crate::{ast::Expr, token::Token};

// fn expr_parser() -> impl Parser<Token, Expr, Error = Simple<Token>> {
// let primary = select! {
// Token::String { value } => Expr::
// };
// todo!()
// }
