use chumsky::primitive::todo;

use crate::{
    ast::{Expr, Op},
    lexer::Span,
    value::Value,
    vm::{chunk::Chunk, opcode::OpCode},
};

pub struct Compiler {
    locals: Vec<Local>,
    scope_depth: usize,
}

/// What's the plan for locals?
/// First for Let expression create a new local with the scope depth
/// When a Block is encountered call 'self.begin_scope()' to increment the 'scope_depth'
/// When the block is done compiling we call `self.end_scope()`
#[derive(Debug, Clone)]
pub struct Local {
    name: String,
    depth: usize,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            locals: Vec::new(),
            scope_depth: 0,
        }
    }
    pub fn compile(&mut self, chunky: &mut Chunk, expr: &Expr) -> CompilerResult<()> {
        match expr {
            Expr::Unit(location) => {
                chunky.write_opcode(OpCode::Unit, &[], location.clone());
                Ok(())
            }
            Expr::Int { value, location } => {
                let raw_int = value.parse::<i64>().unwrap();
                chunky.add_constant(Value::Int(raw_int));

                let index: u8 = (chunky.constants.len() - 1) as u8;
                chunky.write_opcode(OpCode::GetConstant, &[index], location.clone());
                Ok(())
            }
            Expr::Str { value, location } => {
                chunky.add_constant(Value::Str(Box::new(value.to_string())));
                let index: u8 = (chunky.constants.len() - 1) as u8;

                chunky.write_opcode(OpCode::GetConstant, &[index], location.clone());
                Ok(())
            }
            Expr::Bool { value, location } => {
                let raw_bool: bool = value.parse().unwrap();
                chunky.add_constant(Value::Bool(raw_bool));
                let index: u8 = (chunky.constants.len() - 1) as u8;

                chunky.write_opcode(OpCode::GetConstant, &[index], location.clone());
                Ok(())
            }
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
            Expr::Block { expr, location: _ } => self.compile_block(chunky, expr),
            Expr::If {
                condition,
                truthy_branch,
                falsy_branch,
                location,
            } => self.compile_if_else(
                chunky,
                condition,
                truthy_branch,
                falsy_branch,
                location.clone(),
            ),
            _ => todo!(),
        }
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
        self.compile(chunky, lhs)?;
        self.compile(chunky, rhs)?;

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
            .find(|(_, local)| local.name == name)
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
        _location: Span,
    ) -> CompilerResult<()> {
        let local = Local {
            name: name.to_string(),
            depth: self.scope_depth,
        };
        self.locals.push(local);

        // Compile the initializer leaving it at the top of the stack.
        // then compile the next expression
        self.compile(chunky, initializer)?;
        self.compile(chunky, then)?;

        Ok(())
    }

    fn compile_if_else(
        &mut self,
        chunky: &mut Chunk,
        condition: &Expr,
        truthy_branch: &Expr,
        falsy_branch: &Expr,
        location: Span,
    ) -> CompilerResult<()> {
        // Compile the condition leaving it at the top of the stack
        self.compile(chunky, condition)?;

        // Write a dummy jump instruction to the else block
        // Store it's jump location's index to be patched after compiling the if block.
        chunky.write_opcode(OpCode::JumpIfFalse, &[69], location.clone());
        let jump_to_beginning_of_else_index = chunky.code.len() - 1;

        // Compile the truthy branch
        self.compile(chunky, truthy_branch)?;

        // Patch jump_to_else to after the if block
        chunky.patch_instruction(jump_to_beginning_of_else_index, chunky.code.len() as u8);

        // Write a dummy jump instruction to after the else block
        // Store it's jump location's index to be patched after compiling the if block.
        chunky.write_opcode(OpCode::Jump, &[69], location);
        let jump_to_after_else_index = chunky.code.len() - 1;

        // Compile the falsy branch
        self.compile(chunky, falsy_branch)?;

        // Patch jump_to_after_else to after the else block
        chunky.patch_instruction(jump_to_after_else_index, chunky.code.len() as u8);

        Ok(())
    }

    fn compile_block(&mut self, chunky: &mut Chunk, inner_expr: &Expr) -> CompilerResult<()> {
        self.begin_scope();
        self.compile(chunky, inner_expr)?;
        self.end_scope();

        Ok(())
    }

    fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    fn end_scope(&mut self) {
        if self.scope_depth > 0 {
            self.scope_depth -= 1;

            let prev_scope_start = self
                .locals
                .iter()
                .enumerate()
                .find(|(_, local)| local.depth > self.scope_depth);

            if let Some((prev_scope_start, _)) = prev_scope_start {
                self.locals.truncate(prev_scope_start)
            }
        }
    }
}

#[derive(Debug)]
pub enum CompilerErr {
    PlaceHolder,
}

type CompilerResult<T> = Result<T, CompilerErr>;

#[cfg(test)]
mod tests {
    use crate::ast::{Expr, Op};
    use crate::value::Value;
    use crate::vm::chunk::Chunk;

    use super::{Compiler, Local};

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
        Compiler::new()
            .compile(&mut chunky, &expr)
            .expect("Compiler error");

        chunky.disassemble("Compiler result");
    }

    #[test]
    fn end_scope_works() {
        let mut compy = Compiler::new();
        let locals = vec![
            Local {
                name: "a".to_string(),
                depth: 0,
            },
            Local {
                name: "b".to_string(),
                depth: 1,
            },
        ];

        compy.scope_depth = locals.last().unwrap().depth;
        compy.locals = locals;
        compy.end_scope();
        assert_eq!(compy.locals.len(), 1);
    }

    #[test]
    fn end_scope_handles_empty_scopes() {
        let mut compy = Compiler::new();
        let locals = vec![
            Local {
                name: "a".to_string(),
                depth: 0,
            },
            Local {
                name: "b".to_string(),
                depth: 1,
            },
            Local {
                name: "c".to_string(),
                depth: 1,
            },
            Local {
                name: "d".to_string(),
                depth: 3,
            },
        ];

        compy.scope_depth = locals.last().unwrap().depth;
        // Current scope == 3;
        compy.locals = locals;

        // Current scope: 2;
        compy.end_scope();
        assert_eq!(compy.locals.len(), 3);

        // Current scope: 1;
        compy.end_scope();
        assert_eq!(compy.locals.len(), 3);

        // Current scope: 0;
        compy.end_scope();
        assert_eq!(compy.locals.len(), 1);
    }
}
