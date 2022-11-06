#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Tipo {
    App { name: String },
    Fn { args: Vec<Tipo>, ret: Box<Tipo> },
}

impl Tipo {
    pub fn new(name: &str) -> Tipo {
        Tipo::App {
            name: name.to_string(),
        }
    }

    pub fn new_fn(args: Vec<Tipo>, ret: Tipo) -> Tipo {
        Tipo::Fn {
            args,
            ret: Box::new(ret),
        }
    }

    pub fn int_type() -> Tipo {
        Tipo::new("int")
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

    pub fn is_fn(&self) -> bool {
        matches!(self, Tipo::Fn { .. })
    }

    pub fn is_int(&self) -> bool {
        *self == Tipo::int_type()
    }
    pub fn is_string(&self) -> bool {
        *self == Tipo::string_type()
    }

    pub fn is_bool(&self) -> bool {
        *self == Tipo::bool_type()
    }

    pub fn is_unit(&self) -> bool {
        *self == Tipo::unit_type()
    }
}

impl std::fmt::Display for Tipo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Tipo::*;
        match self {
            App { name } => write!(f, "{name}"),
            Fn { args: _, ret: _ } => write!(f, "<function>"),
        }
    }
}
