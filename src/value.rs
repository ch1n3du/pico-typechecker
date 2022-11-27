use crate::function::Function;
use crate::tipo::Tipo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Int(i64),
    Str(Box<String>),
    Bool(bool),
    Unit,
    Fn(Box<Function>),
}

impl Value {
    pub fn get_tipo(&self) -> Tipo {
        use Value::*;
        match self {
            Int(_) => Tipo::int_type(),
            Str(_) => Tipo::string_type(),
            Bool(_) => Tipo::bool_type(),
            Unit => Tipo::unit_type(),
            Fn(f) => f.get_tipo(),
        }
    }

    pub fn logical_and(&self, rhs: &Self) -> Value {
        use Value::*;

        match (self, rhs) {
            (&Bool(b1), &Bool(b2)) => Bool(b1 && b2),
            _ => self.clone(),
        }
    }

    pub fn logical_or(&self, rhs: &Self) -> Value {
        use Value::*;
        match (self, rhs) {
            (&Bool(b1), &Bool(b2)) => Bool(b1 || b2),
            _ => self.clone(),
        }
    }

    pub fn logical_not(&self) -> Value {
        use Value::*;
        match self {
            &Bool(b) => Bool(!b),
            _ => self.clone(),
        }
    }
}

impl std::ops::Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        use Value::*;

        match (&self, rhs) {
            (Int(n1), Int(n2)) => Value::Int(n1 + n2),
            (Str(s1), Str(s2)) => Value::Str(Box::new(format!("{}{}", *s1, *s2))),
            _ => self.clone(),
        }
    }
}

impl std::ops::Sub for Value {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        use Value::*;

        match (&self, rhs) {
            (Int(n1), Int(n2)) => Value::Int(n1 - n2),
            _ => self.clone(),
        }
    }
}

impl std::ops::Div for Value {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        use Value::*;

        match (&self, rhs) {
            (Int(n1), Int(n2)) => Value::Int(n1 / n2),
            _ => self.clone(),
        }
    }
}

impl std::ops::Mul for Value {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        use Value::*;

        match (&self, rhs) {
            (Int(n1), Int(n2)) => Value::Int(n1 * n2),
            _ => self.clone(),
        }
    }
}

impl std::cmp::PartialOrd for Value {
    fn lt(&self, other: &Self) -> bool {
        use Value::*;
        match (self, other) {
            (Int(num_1), Int(num_2)) => num_1 < num_2,
            _ => false,
        }
    }

    fn le(&self, other: &Self) -> bool {
        use Value::*;
        match (self, other) {
            (Int(num_1), Int(num_2)) => num_1 <= num_2,
            _ => false,
        }
    }

    fn gt(&self, other: &Self) -> bool {
        use Value::*;
        match (self, other) {
            (Int(num_1), Int(num_2)) => num_1 > num_2,
            _ => false,
        }
    }

    fn ge(&self, other: &Self) -> bool {
        use Value::*;
        match (self, other) {
            (Int(num_1), Int(num_2)) => num_1 >= num_2,
            _ => false,
        }
    }

    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        None
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Value::*;

        match self {
            Int(n) => write!(f, "{n}"),
            Str(s) => write!(f, "{s}"),
            Bool(b) => write!(f, "{b}"),
            Unit => write!(f, "()"),
            Fn(_f) => todo!(),
        }
    }
}

impl std::default::Default for Value {
    fn default() -> Self {
        Self::Unit
    }
}
