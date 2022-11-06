use crate::tipo::Tipo;

#[derive(Debug, Clone, PartialEq, Eq)]
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
        then: Box<Expr>,
    },
    If {
        condition: Box<Expr>,
        truthy_branch: Box<Expr>,
        falsy_branch: Option<Box<Expr>>,
    },
    Funk {
        name: String,
        fn_: Function,
        // then: Box<Expr>,
    },
    Fn(Function),
    Block(Box<Expr>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub params: Vec<(String, Tipo)>,
    pub ret: Tipo,
    pub body: Box<Expr>,
}

impl Function {
    pub fn get_tipo(&self) -> Tipo {
        let args = self.params.iter().map(|(_, t)| t).cloned().collect();

        Tipo::new_fn(args, self.ret.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Num(i64),
    Str(String),
    Bool(bool),
    Unit,
    Fn(Function),
}

impl Value {
    pub fn get_tipo(&self) -> Tipo {
        use Value::*;
        match self {
            Num(_) => Tipo::int_type(),
            Str(_) => Tipo::string_type(),
            Bool(_) => Tipo::bool_type(),
            Unit => Tipo::unit_type(),
            Fn(_) => todo!(),
        }
    }
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
