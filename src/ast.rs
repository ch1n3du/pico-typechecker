use crate::{function::Function, lexer::Span, tipo::Tipo, value::Value};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    // Literals
    Int {
        value: String,
        location: Span,
    },
    Str {
        value: String,
        location: Span,
    },
    Bool {
        value: String,
        location: Span,
    },
    Unit(Span),
    Identifier {
        value: String,
        location: Span,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
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
    Let {
        name: String,
        let_tipo: Option<Tipo>,
        initializer: Box<Expr>,
        then: Box<Expr>,
        location: Span,
    },
    Block {
        expr: Box<Expr>,
        location: Span,
    },
    If {
        condition: Box<Expr>,
        truthy_branch: Box<Expr>,
        falsy_branch: Box<Expr>,
        location: Span,
    },
    Fn {
        params: Vec<(String, Tipo)>,
        return_tipo: Tipo,
        body: Box<Expr>,
        location: Span,
    },
    Funk {
        name: String,
        params: Vec<(String, Tipo)>,
        return_tipo: Tipo,
        body: Box<Expr>,
        then: Box<Expr>,
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
// EOF
// Ehh