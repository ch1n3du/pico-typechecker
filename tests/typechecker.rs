use pico_typechecker::{
    ast::{Expr, Op},
    typechecker::*,
    value::Value,
};

#[test]
fn dummy() {
    let lhs: Box<Expr> = Box::new(Expr::Value {
        value: Value::Int(12),
        location: 0..1,
    });
    // let rhs: Box<Expr> = Box::new(Expr::Value(Value::Num(18)));
    let rhs: Box<Expr> = Box::new(Expr::Value {
        value: Value::Str(Box::new("Hello".to_string())),
        location: 0..1,
    });

    let testing = Expr::Binary {
        lhs,
        op: Op::Plus,
        rhs,
        location: 0..1,
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
