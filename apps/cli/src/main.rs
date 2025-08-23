use syntax::lang::{BinaryOperation, Expression, Program};

fn main() {
    let prog = Program::new(Expression::let_in(
        "x",
        Expression::bin(
            Expression::lit(12),
            BinaryOperation::Add,
            Expression::lit(20),
        ),
        Expression::bin(
            Expression::lit(10),
            BinaryOperation::Add,
            Expression::var("x"),
        ),
    ));
    println!("{prog}")
}
