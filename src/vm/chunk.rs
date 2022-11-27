use crate::{lexer::Span, value::Value, vm::opcode::OpCode};

pub const SEP: &str = "׀";

#[derive(Debug, Clone)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
    pub lines: Vec<(Span, usize)>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn patch_instruction(&mut self, index: usize, instruction: u8) {
        self.code[index] = instruction;
    }

    /// Gets the instruction at a given index.
    // Returns `None` if thr index is out of bounds
    pub fn get_instruction(&self, index: usize) -> Option<u8> {
        if index >= self.code.len() {
            None
        } else {
            Some(self.code[index])
        }
    }

    /// Gets a constant at a given index
    pub fn get_constant(&self, index: usize) -> Option<&Value> {
        if index >= self.constants.len() {
            // panic!(
            // "Tried to access a constant at index ({index}) out of bounds array {:?}",
            // self.constants
            // )
            None
        } else {
            Some(&self.constants[index])
        }
    }

    /// Writes an OpCode to the Chunk.
    pub fn write_opcode(&mut self, op: OpCode, operands: &[u8], span: Span) {
        if operands.len() != op.arity() {
            panic!("Expected {} operands for {op} OPCode.", op.arity())
        }

        self.write_instruction(op as u8, span.clone());

        for instruction in operands {
            self.write_instruction(*instruction, span.clone())
        }
    }

    /// Writes an Instruction to the Chunk.
    fn write_instruction(&mut self, instruction: u8, current_line: Span) {
        self.code.push(instruction);

        if let Some((last_line, _)) = self.lines.last() {
            if *last_line == current_line {
                // If the current line is on the last line
                self.lines.last_mut().unwrap().1 += 1;
            } else {
                // If `current_line` is not in `lines` `push` it.
                self.lines.push((current_line, 1))
            }
        } else {
            // If `lines` is empty push the current line
            // to `lines`.
            self.lines.push((current_line, 1))
        }
    }

    /// Adds a constant to the chunk.
    pub fn add_constant(&mut self, value: Value) {
        self.constants.push(value)
    }

    /// Returns the line number of an instruction if it's at the beginning of the line.
    pub fn get_line_no(&self, index: usize) -> Option<Span> {
        if index > self.code.len() - 1 {
            None
        } else {
            let mut acc = 0;

            for (line, count) in &self.lines {
                if acc == index {
                    return Some(line.clone());
                } else if acc > index {
                    return None;
                }

                acc += count;
            }
            None
        }
    }

    /// Prints debug information about the chunk.
    pub fn disassemble(&self, name: &str) {
        let top = format!("============== {name} ==============");
        let h_line_thick = "=".repeat(top.len());
        // let h_line_thin = "＿".repeat(top.len());

        println!("{top}");
        println!("Line       {SEP} Raw  {SEP} Repr");
        println!("{}", h_line_thick);

        let mut offset = 0;

        while offset < self.code.len() {
            let op: OpCode = self.code[offset].try_into().expect("Unknown OpCode.");
            offset = op.disassemble(&self, offset);
        }

        println!("{}", h_line_thick);
        println!("CONSTANTS");
        println!("{}", h_line_thick);
        println!("Index {SEP} Constant");
        println!("{}", h_line_thick);
        for (i, con) in self.constants.iter().enumerate() {
            println!("{i:04}  {SEP} {con}")
        }
    }
}

impl std::fmt::Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for op_code in &self.code {
            writeln!(f, "{op_code}")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::vm::opcode::OpCode;

    use super::Chunk;
    // TODO FIX

    #[test]
    fn get_ins_returns_none_when_out_of_bounds() {
        let chunky = Chunk::new();
        let none_ins = chunky.get_instruction(0);

        assert_eq!(none_ins, None)
    }

    #[test]
    fn get_ins_returns_some_when_in_bounds() {
        let mut chunky = Chunk::new();
        chunky.write_opcode(OpCode::GetConstant, &[0], 0..1);
        let ins = chunky.get_instruction(0);

        assert_eq!(ins, Some(1))
    }
}
