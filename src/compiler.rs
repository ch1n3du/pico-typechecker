use crate::{
    ast::{Expr, Op},
    lexer::Span,
    value::Value,
    vm::{chunk::Chunk, opcode::OpCode},
};

pub struct Compiler {
    locals: Vec<Local>,
    scope_depth: usize,
    /// Stores the index of the beginning of the locals for the current scope
    scope_start: usize,
}

/// What's the plan for locals?
/// First for Let expression create a new local with the scope depth
/// When a Block is encountered call 'self.begin_scope()' to increment the 'scope_depth'
/// When the block is done compiling we call `self.end_scope()`
pub struct Local {
    name: String,
    depth: usize,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            locals: Vec::new(),
            scope_depth: 0,
            scope_start: 0,
        }
    }
    pub fn compile(&mut self, chunky: &mut Chunk, expr: &Expr) -> CompilerResult<()> {
        match expr {
            Expr::Value { value, location } => self.compile_value(chunky, value, location.clone()),
            Expr::Unary { op, rhs, location } => {
                self.compiler_unary(chunky, location.clone(), *op, rhs)
            }
            Expr::Binary {
                lhs,
                op,
                rhs,
                location,
            } => self.compile_binary(chunky, location.clone(), *op, lhs, rhs),
            Expr::Grouping { expr, location: _ } => self.compile(chunky, expr),
            Expr::Let {
                name,
                let_tipo: _,
                initializer,
                then,
                location,
            } => self.compile_let(chunky, name, initializer, then, location.clone()),
            Expr::Identifier { value, location } => {
                self.compile_identifier(chunky, &value, location.clone())
            }
            _ => todo!(),
        }
    }

    fn compile_identifier(
        &mut self,
        chunky: &mut Chunk,
        name: &str,
        location: Span,
    ) -> CompilerResult<()> {
        // When an identifier is found find the it's index on the stack.
        // I assume the variable exists as the typechecker already checks for that.
        let (index, _) = self
            .locals
            .iter()
            .enumerate()
            .find(|(index, local)| local.name == name)
            .unwrap();

        chunky.write_opcode(OpCode::GetLocal, &[index as u8], location.clone());
        Ok(())
    }

    fn compile_let(
        &mut self,
        chunky: &mut Chunk,
        name: &str,
        initializer: &Expr,
        then: &Expr,
        location: Span,
    ) -> CompilerResult<()> {
        let local = Local {
            name: name.to_string(),
            depth: self.scope_depth,
        };
        self.locals.push(local);
        let local_index = self.locals.len() - 1;

        // Compile the initializer leaving it at the top of the stack.
        self.compile(chunky, initializer)?;

        Ok(())
    }

    fn compile_value(
        &mut self,
        chunky: &mut Chunk,
        value: &Value,
        location: Span,
    ) -> CompilerResult<()> {
        match value {
            Value::Unit => chunky.write_opcode(OpCode::Unit, &[], location.clone()),
            Value::Bool(true) => chunky.write_opcode(OpCode::True, &[], location.clone()),
            Value::Bool(false) => chunky.write_opcode(OpCode::False, &[], location.clone()),
            value => {
                chunky.add_constant(value.clone());
                let index: u8 = (chunky.constants.len() - 1) as u8;
                chunky.write_opcode(OpCode::GetConstant, &[index], location.clone());
            }
        }
        Ok(())
    }

    fn compiler_unary(
        &mut self,
        chunky: &mut Chunk,
        location: Span,
        op: Op,
        rhs: &Expr,
    ) -> CompilerResult<()> {
        self.compile(chunky, rhs)?;

        let unary_op: OpCode = match op {
            Op::Minus => OpCode::Negate,
            Op::Not => OpCode::LogicalNot,
            _ => todo!(),
        };

        chunky.write_opcode(unary_op, &[], location.clone());
        Ok(())
    }

    fn compile_binary(
        &mut self,
        chunky: &mut Chunk,
        location: Span,
        op: Op,
        lhs: &Expr,
        rhs: &Expr,
    ) -> CompilerResult<()> {
        self.compile(chunky, lhs);
        self.compile(chunky, rhs);

        let bin_opcode: OpCode = match op {
            Op::Plus => OpCode::Add,
            Op::Minus => OpCode::Subtract,
            Op::Multiply => OpCode::Multiply,
            Op::Divide => OpCode::Divide,
            Op::EqualEqual => OpCode::Equal,
            Op::NotEqual => OpCode::NotEqual,
            Op::Less => OpCode::Less,
            Op::LessEqual => OpCode::LessEqual,
            Op::Greater => OpCode::Greater,
            Op::GreaterEqual => OpCode::GreaterEqual,
            Op::And => OpCode::LogicalAnd,
            Op::Or => OpCode::LogicalOr,
            _ => todo!(),
        };

        chunky.write_opcode(bin_opcode, &[], location.clone());
        Ok(())
    }
}

pub enum CompilerErr {
    PlaceHolder,
}

type CompilerResult<T> = Result<T, CompilerErr>;

#[cfg(test)]
mod tests {
    use crate::ast::{Expr, Op};
    use crate::value::Value;
    use crate::vm::chunk::Chunk;

    use super::Compiler;

    #[test]
    fn basic() {
        let expr = Expr::Binary {
            lhs: Box::new(Expr::Value {
                value: Value::Int(1),
                location: 1..1,
            }),
            op: Op::Plus,
            rhs: Box::new(Expr::Value {
                value: Value::Int(2),
                location: 1..1,
            }),
            location: 1..1,
        };

        let mut chunky = Chunk::new();
        Compiler::new().compile(&mut chunky, &expr);

        chunky.disassemble("Compiler result");
        panic!()
    }
}
