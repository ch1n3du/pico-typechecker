use pico_typechecker::{
    ast::{Expr, Op, Value},
    typechecker::*,
};

#[test]
fn dummy() {
    let lhs: Box<Expr> = Box::new(Expr::Value(Value::Num(12)));
    // let rhs: Box<Expr> = Box::new(Expr::Value(Value::Num(18)));
    let rhs: Box<Expr> = Box::new(Expr::Value(Value::Str("Hello".to_string())));

    let testing = Expr::Binary {
        lhs,
        op: Op::Plus,
        rhs,
    };

    let mut checker = TypeChecker::new();
    let res = checker.check_expr(&testing);

    match res {
        Ok(v) => println!("Success: '{v}'"),
        Err(e) => println!("Error: '{e}'"),
    }

    panic!();
    // assert_eq!(res, Tipo::int_type())
}
