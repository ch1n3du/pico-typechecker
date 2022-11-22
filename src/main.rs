use pico_typechecker::{
    value::Value,
    vm::{chunk::Chunk, opcode::OpCode, VM},
};

fn main() {
    let mut chunky = Chunk::new();

    // Set up constants
    chunky.add_constant(Value::Int(8));
    chunky.add_constant(Value::Int(8));

    // OP_CONSTANT 0;
    chunky.write_opcode(OpCode::GetConstant, &[0], 0..1);

    // OP_CONSTANT 0;
    chunky.write_opcode(OpCode::GetConstantLong, &[0, 0, 1], 0..1);

    chunky.write_opcode(OpCode::Multiply, &[], 0..1);

    chunky.write_opcode(OpCode::Return, &[], 1..2);

    // chunky.disassemble("Chunky");

    let mut vm = VM::new(chunky);
    vm.run().expect("Wow, new error");
    println!("{:?}", vm.values);
}
