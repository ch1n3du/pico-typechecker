use chumsky::Parser;
use pico_typechecker::{
    ast::Expr,
    compiler::Compiler,
    lexer::{lexer, Span},
    parser,
    token::Token,
    typechecker::*,
    vm::{chunk::Chunk, opcode::OpCode, VM},
};

fn main() {
    let expr = try_parsing(
        "
                if 1  < 2 {
                    7
                } else {
                    9
                }
    ",
    );
    let mut checker = TypeChecker::new();

    let tipo = checker
        .check_expr(&expr)
        .unwrap_or_else(|e| panic!("Type Error: {e}"));

    println!("Tipo: {tipo}");

    let mut chunky = Chunk::new();
    let mut compiler = Compiler::new();

    compiler.compile(&mut chunky, &expr).unwrap();
    chunky.write_opcode(OpCode::Return, &[], 0..0);

    chunky.disassemble("If/Else test");

    let mut vm = VM::new(chunky);
    vm.run().unwrap();

    println!("VM SNAPSHOT: {vm:?}");
}

fn try_parsing(src: &str) -> Expr {
    let toks: Vec<(Token, Span)> = lexer().parse(src).unwrap();
    parser::expr_parser()
        .parse(chumsky::Stream::from_iter(1..1, toks.into_iter()))
        .unwrap()
}
