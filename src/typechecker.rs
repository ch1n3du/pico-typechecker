use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

fn file_iter(path: &str) -> Result<impl Iterator, std::io::Error> {
    let f = File::open(path)?;
    let reader = BufReader::with_capacity(10, f);
    Ok(reader.lines())
}

use chumsky::primitive::Container;

use crate::{
    ast::{self, Expr, Op, Tipo, Value},
    token::Token,
};
pub struct TypeChecker {
    scopes: Vec<HashMap<String, Tipo>>,
}

impl TypeChecker {
    fn new() -> TypeChecker {
        TypeChecker {
            scopes: vec![HashMap::new()],
        }
    }

    fn check_expr(&mut self, expr: &Expr) -> Result<Tipo, TypeError> {
        match expr {
            Expr::Value(v) => Ok(v.get_tipo()),
            Expr::Grouping(e) => self.check_expr(e),
            Expr::Unary { op, rhs } => {
                let rhs_tipo = self.check_expr(rhs)?;

                match (op, rhs_tipo.name.as_str()) {
                    (Op::Minus, "number") => Ok(rhs_tipo),
                    (Op::Minus, other_type) => Err(TypeError::Basic(format!(
                        "Invalid unary expression can't make a negative '{other_type}'"
                    ))),
                    (Op::Not, "bool") => Ok(rhs_tipo),
                    (Op::Not, other_type) => Err(TypeError::Basic(format!(
                        "Invalid unary expression can't not '{other_type}'"
                    ))),
                    _ => todo!(),
                }
            }
            Expr::Binary { lhs, op, rhs } => {
                // let args = (self.check_expr(lhs)?, self.check_expr(rhs));
                let lhs_tipo = self.check_expr(lhs)?;
                let rhs_tipo = self.check_expr(rhs)?;
                // let valid_pairs = self.bin_ops.get(op).unwrap().contains();
                match (op, lhs_tipo.name.as_str(), rhs_tipo.name.as_str()) {
                    (Op::Plus, "number", "number") => Ok(lhs_tipo),
                    (Op::Plus, t1, t2) => Err(TypeError::Basic(format!(
                        "Can't apply addition to types '{t1}' and '{t2}'"
                    ))),
                    (Op::Minus, "number", "number") => Ok(lhs_tipo),
                    (Op::Minus, t1, t2) => Err(TypeError::Basic(format!(
                        "Can't apply subtraction to types '{t1}' and '{t2}'"
                    ))),
                    (Op::Multiply, "number", "number") => Ok(lhs_tipo),
                    (Op::Multiply, t1, t2) => Err(TypeError::Basic(format!(
                        "Can't apply multiplication to types '{t1}' and '{t2}'"
                    ))),
                    (Op::Divide, "number", "number") => Ok(lhs_tipo),
                    (Op::Divide, t1, t2) => Err(TypeError::Basic(format!(
                        "Can't apply division to types '{t1}' and '{t2}'"
                    ))),
                    (Op::And, "bool", "bool") => Ok(lhs_tipo),
                    (Op::And, t1, t2) => Err(TypeError::Basic(format!(
                        "Can't apply logical and to types '{t1}' and '{t2}'"
                    ))),
                    (Op::Or, "bool", "bool") => Ok(lhs_tipo),
                    (Op::Or, t1, t2) => Err(TypeError::Basic(format!(
                        "Can't apply logical or to types '{t1}' and '{t2}'"
                    ))),
                    _ => todo!(),
                }
            }
            Expr::Let {
                name,
                tipo,
                initializer,
            } => {
                let var_tipo = match (tipo, initializer) {
                    (None, e) => self.check_expr(e)?,
                    (Some(expected_tipo), initializer) => {
                        let initializer_tipo = self.check_expr(initializer)?;
                        if initializer_tipo != *expected_tipo {
                            return Err(TypeError::Basic(format!("Variable '{name}' expected to have type '{expected_tipo:?}' but got '{initializer_tipo:?}'")));
                        }
                        initializer_tipo
                    }
                };

                self.set_var_tipo(name, var_tipo);
                Ok(Tipo::new("__null__"))
            }

            Expr::Identifier(ident) => self.get_var_tipo(&ident),
            Expr::Block(exprs) => {
                if let Some(last_expr) = exprs.last() {
                    self.check_expr(last_expr)
                } else {
                    Ok(Tipo::new("__null__"))
                }
            }
            Expr::If {
                condition,
                truthy_branch,
                falsy_branch,
            } => {
                if self.check_expr(&condition)?.name == "bool" {
                    return Err(TypeError::Basic(
                        "The condition of an if statement must be a boolean expression".to_string(),
                    ));
                }

                let truthy_tipo = self.check_expr(truthy_branch)?;
                let falsy_tipo = if let Some(falsy_block) = falsy_branch {
                    self.check_expr(&falsy_block)?
                } else {
                    Tipo::new("__null__")
                };

                if truthy_tipo != falsy_tipo {
                    Err(TypeError::Basic(format!(
                        "If('{truthy_tipo:?}') and else('{falsy_tipo:?}') branch has different types."
                    )))
                } else {
                    Ok(truthy_tipo)
                }
            }
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
}

#[derive(Debug, PartialEq, Eq)]
enum TypeError {
    Basic(String),
    VarDoesntExist(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dummy() {
        let lhs: Box<Expr> = Box::new(Expr::Value(Value::Num(12)));
        let rhs: Box<Expr> = Box::new(Expr::Value(Value::Num(18)));

        let testing = Expr::Binary {
            lhs,
            op: Op::Plus,
            rhs,
        };
        let mut checker = TypeChecker::new();
        let res = checker.check_expr(&testing).unwrap();
        assert_eq!(&res.name, "number")
    }
}
