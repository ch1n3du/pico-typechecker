/// This module is the core of the TypeChecker it's a huge match statement on the AST.
use std::collections::HashMap;

use crate::{
    ast::{Expr, Op},
    function::Function,
    tipo::Tipo,
};
pub struct TypeChecker {
    scopes: Vec<HashMap<String, Tipo>>,
}

impl TypeChecker {
    pub fn new() -> TypeChecker {
        TypeChecker {
            scopes: vec![HashMap::new()],
        }
    }

    pub fn check_expr(&mut self, expr: &Expr) -> TypeResult<Tipo> {
        match expr {
            Expr::Identifier { value, .. } => self.get_var_tipo(&value),
            Expr::Value { value, .. } => Ok(value.get_tipo()),
            Expr::Grouping { expr, .. } => self.check_expr(&expr),
            Expr::Unary { op, rhs, .. } => self.check_unary_expr(op.clone(), rhs),
            Expr::Binary { lhs, op, rhs, .. } => self.check_binary_expr(op.clone(), lhs, rhs),
            Expr::Let {
                name,
                let_tipo,
                initializer,
                then,
                ..
            } => self.check_let_expr(name, let_tipo, initializer, then),
            Expr::If {
                condition,
                truthy_branch,
                falsy_branch,
                ..
            } => self.check_if_expr(condition, truthy_branch, falsy_branch),
            Expr::Block { expr, .. } => self.check_expr(&expr),
            Expr::Fn { fn_, .. } => self.check_funk(fn_),
            Expr::Funk { name, fn_, .. } => {
                // Put the expected function type in the scope to handle recursive functions
                let expected_tipo = fn_.get_tipo();
                self.set_var_tipo(name, expected_tipo);

                let tipo = self.check_funk(fn_)?;
                self.set_var_tipo(name, tipo.clone());
                Ok(tipo)
            }
        }
    }

    /// There's a small bug with using variables in local scopes
    fn check_funk(&mut self, funk: &Function) -> TypeResult<Tipo> {
        self.begin_scope();

        let Function { params, ret, body } = funk;
        let mut tipo_params: Vec<Tipo> = Vec::new();

        for (name, tipo) in params {
            self.set_var_tipo(name, tipo.clone());
            tipo_params.push(tipo.clone());
        }

        self.end_scope();

        let actual_ret = self.check_expr(body)?;

        if actual_ret == *ret {
            Ok(Tipo::new_fn(tipo_params, ret.clone()))
        } else {
            Err(TypeError::Basic(format!(
                "Expected return type of {ret}, got {actual_ret}."
            )))
        }
    }

    fn check_if_expr(
        &mut self,
        condition: &Expr,
        truthy_branch: &Expr,
        falsy_branch: &Option<Box<Expr>>,
    ) -> TypeResult<Tipo> {
        if !self.check_expr(condition)?.is_bool() {
            return Err(TypeError::Basic(
                "If/Else condition must be a boolean expression".to_string(),
            ));
        }

        let truthy_tipo = self.check_expr(truthy_branch)?;
        if let Some(falsy) = falsy_branch {
            let falsy_tipo = self.check_expr(falsy)?;
            if falsy_tipo != truthy_tipo {
                return Err(TypeError::Basic(
                    "Truthy and falsy branch in an if/else expression must have the same type."
                        .to_string(),
                ));
            }

            Ok(truthy_tipo)
        } else {
            if truthy_tipo.is_unit() {
                Err(TypeError::Basic(
                    "Truthy and falsy branch in an if/else expression must have the same type."
                        .to_string(),
                ))
            } else {
                Ok(Tipo::unit_type())
            }
        }
    }

    fn check_let_expr(
        &mut self,
        name: &str,
        tipo: &Option<Tipo>,
        initializer: &Expr,
        then: &Expr,
    ) -> TypeResult<Tipo> {
        let init_tipo = self.check_expr(initializer)?;
        let tipo = if let Some(t) = tipo {
            if init_tipo != *t {
                return Err(TypeError::Basic(format!(
                    "Var '{name}' was expected to be of type {t}, got {init_tipo}."
                )));
            }
            t.clone()
        } else {
            init_tipo
        };

        self.set_var_tipo(name, tipo);
        self.check_expr(then)
    }

    fn check_unary_expr(&mut self, op: Op, rhs: &Expr) -> TypeResult<Tipo> {
        let rhs_tipo = self.check_expr(rhs)?;

        match (op, rhs_tipo) {
            // - t1: int -> int
            (Op::Minus, t1) => {
                if t1.is_int() {
                    Ok(Tipo::int_type())
                } else {
                    Err(TypeError::Unary { op, t1 })
                }
            }
            // not t1: bool -> bool
            (Op::Not, t1) => {
                if t1.is_int() {
                    Ok(Tipo::int_type())
                } else {
                    Err(TypeError::Unary { op, t1 })
                }
            }
            (op, t1) => Err(TypeError::Unary { op, t1 }),
        }
    }

    fn check_binary_expr(&mut self, op: Op, lhs: &Expr, rhs: &Expr) -> TypeResult<Tipo> {
        let lhs_tipo = self.check_expr(lhs)?;
        let rhs_tipo = self.check_expr(rhs)?;

        match (op, lhs_tipo, rhs_tipo) {
            // ARITHMETIC OPERATIONS

            // t1: int + t2: int -> int
            // t1: string + t2: string -> string
            (Op::Plus, t1, t2) => {
                if t1.is_int() && t2.is_int() {
                    Ok(Tipo::int_type())
                } else if t1.is_string() && t2.is_string() {
                    Ok(Tipo::string_type())
                } else {
                    Err(TypeError::Binary { op, t1, t2 })
                }
            }

            // int - int -> int
            (Op::Minus, t1, t2) => {
                if t1.is_int() && t2.is_int() {
                    Ok(Tipo::int_type())
                } else {
                    Err(TypeError::Binary { op, t1, t2 })
                }
            }
            // int * int -> int
            (Op::Multiply, t1, t2) => {
                if t1.is_int() && t2.is_int() {
                    Ok(Tipo::int_type())
                } else {
                    Err(TypeError::Binary { op, t1, t2 })
                }
            }
            // int / int -> int
            (Op::Divide, t1, t2) => {
                if t1.is_int() && t2.is_int() {
                    Ok(Tipo::int_type())
                } else {
                    Err(TypeError::Binary { op, t1, t2 })
                }
            }

            // COMPARISON OPERATOR: ==, !=, <, <=, >, >= ;

            // t1: T == t2: T -> bool
            (Op::EqualEqual, t1, t2) => {
                if t1 == t2 {
                    Ok(Tipo::bool_type())
                } else {
                    Err(TypeError::Binary { op, t1, t2 })
                }
            }

            // t1: T == t2: T -> bool
            (Op::NotEqual, t1, t2) => {
                if t1 == t2 {
                    Ok(Tipo::bool_type())
                } else {
                    Err(TypeError::Binary { op, t1, t2 })
                }
            }

            // t1: int < t2: int -> int ;
            (Op::Less, t1, t2) => {
                if t1.is_int() && t2.is_int() {
                    Ok(Tipo::int_type())
                } else {
                    Err(TypeError::Binary { op, t1, t2 })
                }
            }
            // int <= int -> int
            (Op::LessEqual, t1, t2) => {
                if t1.is_int() && t2.is_int() {
                    Ok(Tipo::int_type())
                } else {
                    Err(TypeError::Binary { op, t1, t2 })
                }
            }
            // t1: int > t2: int -> int
            (Op::Greater, t1, t2) => {
                if t1.is_int() && t2.is_int() {
                    Ok(Tipo::int_type())
                } else {
                    Err(TypeError::Binary { op, t1, t2 })
                }
            }
            // int <= int -> int
            (Op::GreaterEqual, t1, t2) => {
                if t1.is_int() && t2.is_int() {
                    Ok(Tipo::int_type())
                } else {
                    Err(TypeError::Binary { op, t1, t2 })
                }
            }

            // BOOLEAN OPERATIONS
            // =====================

            // t1: bool && t2: bool -> bool
            (Op::And, t1, t2) => {
                if t1.is_bool() && t2.is_bool() {
                    Ok(Tipo::int_type())
                } else {
                    Err(TypeError::Basic(format!(
                        "Can't apply 'logical and' to types '{t1}' and '{t2}'"
                    )))
                }
            }

            // t1: bool || t2: bool -> bool
            (Op::Or, t1, t2) => {
                if t1.is_bool() && t2.is_bool() {
                    Ok(Tipo::int_type())
                } else {
                    Err(TypeError::Binary { op, t1, t2 })
                }
            }

            (op, t1, t2) => Err(TypeError::Binary { op, t1, t2 }),
        }
    }

    fn get_var_tipo(&self, name: &str) -> Result<Tipo, TypeError> {
        for scope in self.scopes.iter().rev() {
            if let Some(tipo) = scope.get(name) {
                return Ok(tipo.clone());
            }
        }
        Err(TypeError::VarDoesntExist(name.to_string()))
    }

    fn set_var_tipo(&mut self, name: &str, tipo: Tipo) {
        if let Some(top_scope) = self.scopes.last_mut() {
            top_scope.insert(name.to_string(), tipo.clone());
        }
        ()
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new())
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }
}

pub type TypeResult<T> = Result<T, TypeError>;

#[derive(Debug, PartialEq, Eq)]
pub enum TypeError {
    Basic(String),
    VarDoesntExist(String),
    Unary { op: Op, t1: Tipo },
    Binary { op: Op, t1: Tipo, t2: Tipo },
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TypeError::*;
        match self {
            Basic(s) => write!(f, "{s}"),
            VarDoesntExist(name) => write!(f, "Variable {name} doesn't exist"),
            Unary { op, t1 } => write!(f, "Can't apply unary operation '{op}' to type '{t1}'"),
            Binary { op, t1, t2 } => write!(
                f,
                "Can't apply binary operation '{op}' to types '{t1}' and '{t2}'"
            ),
        }
    }
}
