use crate::token::Token;

pub enum Expr {
    /// Variable identifier like "x".
    Identifier(String),
    Value(Value),
    Grouping(Box<Expr>),
    Unary {
        op: Token,
        rhs: Box<Expr>,
    },
    Binary {
        lhs: Box<Expr>,
        op: Token,
        rhs: Box<Expr>,
    },
    If {
        predicate: Box<Expr>,
        truthy_branch: Box<Expr>,
        falsy_branch: Option<Box<Expr>>,
    },
    Assignment {
        name: String,
        value: Box<Expr>,
    },
    Let {
        name: String,
        tipo: Option<Tipo>,
        initializer: Option<Box<Expr>>,
    },
    Block(Vec<Expr>),
    While {
        condition: Box<Expr>,
        body: Box<Expr>,
    },
    Ignore(Box<Expr>),
}

pub enum Stmt {
    If(Expr),
}

pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Unit,
}

impl Value {
    pub fn get_tipo(&self) -> Tipo {
        use Value::*;
        match self {
            Number(_) => Tipo::new("number"),
            String(_) => Tipo::new("string"),
            Bool(_) => Tipo::new("bool"),
            Unit => Tipo::new("__unit__"),
        }
    }

    pub fn number_type() -> Tipo {
        Tipo::new("number")
    }

    pub fn string_type() -> Tipo {
        Tipo::new("string")
    }

    pub fn bool_type() -> Tipo {
        Tipo::new("bool")
    }

    pub fn unit_type() -> Tipo {
        Tipo::new("__unit__")
    }
}

pub enum Op {
    Plus,
    Minus,
    Divide,
    Multiply,
    And,
    Or,
    Not,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tipo {
    pub name: String,
}

impl Tipo {
    pub fn new(name: &str) -> Tipo {
        Tipo {
            name: name.to_string(),
        }
    }
}
