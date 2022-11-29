use super::chunk::{Chunk, SEP};
use super::Value;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum OpCode {
    /// Stops execution.
    Return = 0,

    /// Reads the next byte in the instruction stream as an index, pushes the constant
    /// in the  chunk at that index to the top of the stack.
    GetConstant = 1,

    /// Works like `GetConstant` but takes three bytes as operands allowing for larger indexing.
    GetConstantLong = 2,

    /// Numeric OpCodes
    Negate = 3,
    Add = 4,
    Subtract = 5,
    Multiply = 6,
    Divide = 7,

    /// Comparison OpCodes
    Equal = 8,
    NotEqual = 9,
    Less = 10,
    LessEqual = 11,
    Greater = 12,
    GreaterEqual = 13,
    LogicalAnd = 14,
    LogicalOr = 15,
    LogicalNot = 16,

    /// Value Constant OpCodes
    Unit = 17,
    True = 18,
    False = 19,

    /// Locals Manipulation OpCodes
    SetLocal = 20,
    GetLocal = 21,

    /// Stack Manipulation OpCodes
    Pop = 22,
    PopN = 23,

    /// Jump Instructions
    Jump = 24,
    JumpIfTrue = 25,
    JumpIfFalse = 26,
}

impl OpCode {
    /// Pretty prints info on the Opcode
    pub fn disassemble(&self, chunk: &Chunk, offset: usize) -> usize {
        let end_offset = offset + self.arity();

        if end_offset > chunk.code.len() - 1 {
            panic!(
                "Error: Expected {} more instruction(s).",
                end_offset - (chunk.code.len() - 1)
            );
        }

        use OpCode::*;
        let repr = match self {
            GetConstant => {
                let index = chunk.code[offset + 1];
                let constant: &Value = chunk.get_constant(index as usize).unwrap();

                Some(format!(" [{index}] -> {constant}"))
            }
            GetConstantLong => {
                let p1 = chunk.code[offset + 1];
                let p2 = chunk.code[offset + 2];
                let p3 = chunk.code[offset + 3];
                let index: u32 = u32::from_be_bytes([0, p1, p2, p3]);

                let constant: &Value = chunk.get_constant(index as usize).unwrap();

                Some(format!(" [{index}] -> {constant}"))
            }
            SetLocal => {
                let index = chunk.code[offset + 1];

                Some(format!(" {index}"))
            }
            GetLocal => {
                let index = chunk.code[offset + 1];

                Some(format!(" {index}"))
            }
            Jump => {
                let index = chunk.code[offset + 1];

                Some(format!(" {index}"))
            }
            JumpIfTrue => {
                let index = chunk.code[offset + 1];

                Some(format!(" {index}"))
            }
            JumpIfFalse => {
                let index = chunk.code[offset + 1];

                Some(format!(" {index}"))
            }
            _ => None,
        };

        let repr = repr.unwrap_or("".to_string());
        let line = chunk
            .get_line_no(offset)
            .map(|s| format!("{s:04?}"))
            .unwrap_or("...".to_string());

        println!("{line} {SEP} {self} {repr}");

        end_offset + 1
    }

    /// Returns the number or operands taken by a given OpCode.
    pub fn arity(&self) -> usize {
        use OpCode::*;

        match self {
            Return => 0,
            Pop => 0,
            PopN => 1,
            GetConstant => 1,
            SetLocal => 1,
            GetLocal => 1,
            GetConstantLong => 3,
            Jump => 1,
            JumpIfTrue => 1,
            JumpIfFalse => 1,

            // Binary OpCodes
            Negate | Add | Subtract | Multiply | Divide | Equal | NotEqual | Less | LessEqual
            | Greater | GreaterEqual | LogicalAnd | LogicalOr | LogicalNot => 0,

            // Constant OpCodes
            Unit | True | False => 0,
        }
    }
}

impl std::fmt::Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04} {SEP} {:?}", *self as u8, self)
    }
}

impl TryFrom<u8> for OpCode {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let res = match value {
            0 => OpCode::Return,
            1 => OpCode::GetConstant,
            2 => OpCode::GetConstantLong,
            3 => OpCode::Negate,
            4 => OpCode::Add,
            5 => OpCode::Subtract,
            6 => OpCode::Multiply,
            7 => OpCode::Divide,
            8 => OpCode::Equal,
            9 => OpCode::NotEqual,
            10 => OpCode::Less,
            11 => OpCode::LessEqual,
            12 => OpCode::Greater,
            13 => OpCode::GreaterEqual,
            14 => OpCode::LogicalAnd,
            15 => OpCode::LogicalOr,
            16 => OpCode::LogicalNot,
            17 => OpCode::Unit,
            18 => OpCode::True,
            19 => OpCode::False,

            20 => OpCode::SetLocal,
            21 => OpCode::GetLocal,

            22 => OpCode::Pop,
            23 => OpCode::PopN,

            24 => OpCode::Jump,
            25 => OpCode::JumpIfTrue,
            26 => OpCode::JumpIfFalse,

            _ => return Err("Invalid OpCode".to_string()),
        };

        Ok(res)
    }
}
