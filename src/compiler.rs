use crate::{
    ast::{Expr, Op},
    value::Value,
    vm::{chunk::Chunk, opcode::OpCode},
};

pub fn compile(chunky: &mut Chunk, expr: &Expr) {
    match expr {
        Expr::Value { value, location } => match value {
            Value::Unit => chunky.write_opcode(OpCode::Unit, &[], location.clone()),
            Value::Bool(true) => chunky.write_opcode(OpCode::True, &[], location.clone()),
            Value::Bool(false) => chunky.write_opcode(OpCode::False, &[], location.clone()),
            value => {
                chunky.add_constant(value);
                let index: u8 = (chunky.constants.len() - 1) as u8;
                chunky.write_opcode(OpCode::GetConstant, &[index], location.clone());
            }
        },
        Expr::Unary { op, rhs, location } => {
            compile(chunky, rhs);

            let unary_op: OpCode = match op {
                Op::Minus => OpCode::Negate,
                Op::Not => OpCode::LogicalNot,
                _ => todo!(),
            };

            chunky.write_opcode(unary_op, &[], location.clone());
        }
        Expr::Binary {
            lhs,
            op,
            rhs,
            location,
        } => {
            compile(chunky, lhs);
            compile(chunky, rhs);

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
        }
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Expr, Op};
    use crate::value::Value;
    use crate::vm::chunk::Chunk;

    use super::compile;

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
        compile(&mut chunky, &expr)
    }
}
