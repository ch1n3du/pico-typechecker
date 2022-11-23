use crate::{function::Function, lexer::Span, tipo::Tipo, value::Value};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    /// Variable identifier like "x".
    Identifier {
        value: String,
        location: Span,
    },
    Value {
        value: Value,
        location: Span,
    },
    Grouping {
        expr: Box<Expr>,
        location: Span,
    },
    Unary {
        op: Op,
        rhs: Box<Expr>,
        location: Span,
    },
    Binary {
        lhs: Box<Expr>,
        op: Op,
        rhs: Box<Expr>,
        location: Span,
    },
    Fn {
        fn_: Function,
        location: Span,
    },
    Let {
        name: String,
        let_tipo: Option<Tipo>,
        initializer: Box<Expr>,
        then: Box<Expr>,
        location: Span,
    },
    If {
        condition: Box<Expr>,
        truthy_branch: Box<Expr>,
        falsy_branch: Option<Box<Expr>>,
        location: Span,
    },
    Funk {
        name: String,
        fn_: Function,
        then: Box<Expr>,
        location: Span,
    },
    Block {
        expr: Box<Expr>,
        location: Span,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Plus,
    Minus,
    Divide,
    Multiply,
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    Not,
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Op::*;
        match self {
            Plus => write!(f, "+"),
            Minus => write!(f, "-"),
            Multiply => write!(f, "*"),
            Divide => write!(f, "/"),
            EqualEqual => write!(f, "=="),
            NotEqual => write!(f, "!="),
            Less => write!(f, "<"),
            LessEqual => write!(f, "<="),
            Greater => write!(f, ">"),
            GreaterEqual => write!(f, ">="),
            And => write!(f, "&&"),
            Or => write!(f, "||"),
            Not => write!(f, "!"),
        }
    }
}
