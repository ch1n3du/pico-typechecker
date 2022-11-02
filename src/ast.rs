#[derive(Debug, Clone)]
pub enum Expr {
    /// Variable identifier like "x".
    Identifier(String),
    Value(Value),
    Grouping(Box<Expr>),
    Unary {
        op: Op,
        rhs: Box<Expr>,
    },
    Binary {
        lhs: Box<Expr>,
        op: Op,
        rhs: Box<Expr>,
    },
    Let {
        name: String,
        tipo: Option<Tipo>,
        initializer: Box<Expr>,
        // then: Box<Expr>,
    },
    If {
        condition: Box<Expr>,
        truthy_branch: Box<Expr>,
        falsy_branch: Option<Box<Expr>>,
    },
    Block(Vec<Expr>),
    // Call {
    // callee: Box<Expr>,
    // args: Vec<Expr>,
    // },
}

pub enum Stmt {
    If(Expr),
}

#[derive(Debug, Clone)]
pub enum Value {
    Num(i64),
    Str(String),
    Bool(bool),
    Unit,
}

impl Value {
    pub fn get_tipo(&self) -> Tipo {
        use Value::*;
        match self {
            Num(_) => Tipo::new("number"),
            Str(_) => Tipo::new("string"),
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

#[derive(Debug, Clone)]
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
