pub mod chunk;
pub mod opcode;

use crate::value::Value;

use self::{chunk::Chunk, opcode::OpCode};

#[derive(Debug)]
pub struct VM {
    pub chunk: Chunk,
    pub ip: usize,
    pub values: Vec<Value>,
}

type BinaryStackOp = fn(Value, Value) -> Value;
type UnaryStackOp = fn(Value) -> Value;

impl VM {
    /// Set's a chunks as the VM's chunk field
    pub fn new(chunk: Chunk) -> VM {
        VM {
            chunk,
            ip: 0,
            values: Vec::new(),
        }
    }

    /// Reads a byte from the `chunk` as the instruction
    /// Converts that byte into an `OpCode` and dispatches it.
    pub fn run(&mut self) -> Result<(), RuntimeErr> {
        use OpCode::*;
        loop {
            let opcode: OpCode = self.read_opcode()?;

            match opcode {
                Return => return Ok(()),
                GetConstant => self.op_constant(),
                GetConstantLong => self.op_constant_long(),
                SetLocal => self.set_local(),
                GetLocal => self.get_local(),
                Pop => {
                    self.pop()?;
                    Ok(())
                }
                PopN => self.pop_n(),

                // Constant OpCodes
                Unit => self.push(Value::Unit),
                True => self.push(Value::Bool(true)),
                False => self.push(Value::Bool(false)),

                // Arithmetic OpCodes
                Negate => self.binary_stack_op(|a, b| a + b),
                Add => self.binary_stack_op(|a, b| a + b),
                Subtract => self.binary_stack_op(|a, b| a - b),
                Multiply => self.binary_stack_op(|a, b| a * b),
                Divide => self.binary_stack_op(|a, b| a / b),

                // Comparison OpCodes
                Equal => self.binary_stack_op(|a, b| Value::Bool(a == b)),
                NotEqual => self.binary_stack_op(|a, b| Value::Bool(a != b)),
                Less => self.binary_stack_op(|a, b| Value::Bool(a < b)),
                LessEqual => self.binary_stack_op(|a, b| Value::Bool(a <= b)),
                Greater => self.binary_stack_op(|a, b| Value::Bool(a > b)),
                GreaterEqual => self.binary_stack_op(|a, b| Value::Bool(a >= b)),

                // Logic OpCodes
                LogicalNot => self.unary_stack_op(|a| a.logical_not()),
                LogicalAnd => self.binary_stack_op(|a, b| a.logical_and(&b)),
                LogicalOr => self.binary_stack_op(|a, b| a.logical_or(&b)),

                // Jump OpCodes
                Jump => self.jump(),
                JumpIfTrue => self.jump_if_true(),
                JumpIfFalse => self.jump_if_false(),
            }?;
        }
    }

    fn op_constant(&mut self) -> Result<(), RuntimeErr> {
        let index: usize = self.read_byte()? as usize;
        let constant = self.get_constant(index)?;

        self.push(constant)?;

        Ok(())
    }

    fn op_constant_long(&mut self) -> Result<(), RuntimeErr> {
        // Read the next three bytes to get the index
        let i1: u8 = self.read_byte()?;
        let i2: u8 = self.read_byte()?;
        let i3: u8 = self.read_byte()?;

        let index = u32::from_be_bytes([0, i1, i2, i3]) as usize;
        let constant = self.get_constant(index)?;

        self.push(constant)?;

        Ok(())
    }

    /// SET_LOCAL index
    fn set_local(&mut self) -> Result<(), RuntimeErr> {
        // Read the next byte as the index
        let local_index = self.read_byte()? as usize;
        let popped_stack_top = self.pop()?;

        // Set the value popped from the top of the stack to the local.
        self.values[local_index] = popped_stack_top;

        Ok(())
    }

    /// GET_LOCAL index
    fn get_local(&mut self) -> Result<(), RuntimeErr> {
        // Read the next byte as the index
        let local_index = self.read_byte()? as usize;

        // Push the local at the given index to the top of the value stack
        self.push(self.values[local_index].clone())
    }

    /// Reads the byte at the index pointed at by the VM's `ip` field.
    /// Increments the VM instruction pointer.
    fn read_byte(&mut self) -> RuntimeResult<u8> {
        let byte = self.chunk.get_instruction(self.ip);
        self.ip += 1;

        if let Some(b) = byte {
            Ok(b)
        } else {
            Err(RuntimeErr::OutOfInstructions(self.ip))
        }
    }

    // Reads an opcode from the chunk
    fn read_opcode(&mut self) -> RuntimeResult<OpCode> {
        let raw_opcode = self.read_byte()?;

        // Try to convert the byte to an OpCode
        raw_opcode
            .try_into()
            .map_err(|_| RuntimeErr::InvalidOpCode(raw_opcode))
    }

    // TODO Change 'get_constant' to return an Option<Value> and map to an error
    fn get_constant(&self, index: usize) -> RuntimeResult<Value> {
        if let Some(c) = self.chunk.get_constant(index) {
            Ok(c.clone())
        } else {
            Err(RuntimeErr::RuntimeErr("Constant out of bounds".to_string()))
        }
    }

    /// POP_N count
    /// Takes one operand(n) and pops (n) number of values from the stack.
    fn pop_n(&mut self) -> RuntimeResult<()> {
        let to_be_popped = self.read_byte()? as usize;

        if self.values.len() >= to_be_popped {
            self.values.truncate(self.values.len() - to_be_popped);
            Ok(())
        } else {
            Err(RuntimeErr::StackTooShort)
        }
    }

    fn jump(&mut self) -> RuntimeResult<()> {
        let destination = self.read_byte()? as usize;
        self.ip = destination;
        Ok(())
    }

    fn jump_if_true(&mut self) -> RuntimeResult<()> {
        let destination = self.read_byte()? as usize;
        if let Value::Bool(true) = self.pop()? {
            self.ip = destination;
        };
        Ok(())
    }

    fn jump_if_false(&mut self) -> RuntimeResult<()> {
        let destination = self.read_byte()? as usize;
        if let Value::Bool(true) = self.pop()? {
            self.ip = destination;
        };
        Ok(())
    }

    fn unary_stack_op(&mut self, f: UnaryStackOp) -> RuntimeResult<()> {
        let a = self.pop()?;

        self.push(f(a))?;

        Ok(())
    }

    fn binary_stack_op(&mut self, f: BinaryStackOp) -> RuntimeResult<()> {
        let a = self.pop()?;
        let b = self.pop()?;

        self.push(f(a, b))?;

        Ok(())
    }

    /// Pushes a value to the `values` stack or returns an `RuntimeErr` if it exceeds the VM::STACK_MAX.
    fn push(&mut self, value: Value) -> RuntimeResult<()> {
        self.values.push(value);
        Ok(())
    }

    /// Pops a value from the Value stack or returns an `RuntimeErr`.
    fn pop(&mut self) -> RuntimeResult<Value> {
        if let Some(value) = self.values.pop() {
            Ok(value)
        } else {
            Err(RuntimeErr::StackTooShort)
        }
    }
}

#[derive(Debug)]
pub enum RuntimeErr {
    CompileErr(String),
    RuntimeErr(String),
    StackOverflow,
    StackTooShort,
    OutOfInstructions(usize),
    InvalidOpCode(u8),
}

pub type RuntimeResult<T> = Result<T, RuntimeErr>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_constant_works() {
        let mut chunky = Chunk::new();

        // Set up constants
        chunky.add_constant(Value::Int(42));

        // OP_CONSTANT 0;
        chunky.write_opcode(OpCode::GetConstant, &[0], 0..1);
        chunky.write_opcode(OpCode::Return, &[], 0..0);

        let mut vm = VM::new(chunky);

        vm.run().expect("Unexpected VM error.");

        assert_eq!(vm.values[0], Value::Int(42))
    }

    #[test]
    fn op_constant_long_works() {
        let mut chunky = Chunk::new();

        // Set up constants
        chunky.add_constant(Value::Int(42));
        chunky.add_constant(Value::Int(69));

        // OP_CONSTANT 0;
        chunky.write_opcode(OpCode::GetConstantLong, &[0, 0, 1], 0..1);

        // OP_RETURN;
        chunky.write_opcode(OpCode::Return, &[], 0..1);

        let mut vm = VM::new(chunky);

        vm.run().unwrap();

        assert_eq!(vm.values[0], Value::Int(69))
    }
}
