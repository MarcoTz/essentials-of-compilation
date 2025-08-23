use super::{Atm, Exp, Program, Stmt, Tail, Var};
use std::{collections::HashMap, fmt};

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    Int,
}

pub trait Typecheck: Sized {
    fn check(&self) -> HashMap<Var, Type>;
}

pub fn typecheck(prog: &mut Program) {
    let vars = prog.check();
    prog.types = vars;
}

impl Typecheck for Program {
    fn check(&self) -> HashMap<Var, Type> {
        let mut vars = HashMap::new();
        for (_, exp) in self.blocks.iter() {
            vars.extend(exp.check());
        }
        vars
    }
}

impl Typecheck for Tail {
    fn check(&self) -> HashMap<Var, Type> {
        match self {
            Tail::Return(e) => e.check(),
            Tail::Seq(stmt, tl) => {
                let mut vars = stmt.check();
                vars.extend(tl.check());
                vars
            }
        }
    }
}

impl Typecheck for Exp {
    fn check(&self) -> HashMap<Var, Type> {
        match self {
            Exp::Atm(a) => a.check(),
            Exp::Read => HashMap::new(),
            Exp::UnaryOp { op: _, exp } => exp.check(),
            Exp::BinOp { exp1, op: _, exp2 } => {
                let mut vars = exp1.check();
                vars.extend(exp2.check());
                vars
            }
        }
    }
}

impl Typecheck for Stmt {
    fn check(&self) -> HashMap<Var, Type> {
        match self {
            Stmt::Assign { var, exp } => {
                let mut vars = exp.check();
                vars.insert(var.clone(), Type::Int);
                vars
            }
        }
    }
}

impl Typecheck for Atm {
    fn check(&self) -> HashMap<Var, Type> {
        match self {
            Atm::Int(_) => HashMap::new(),
            Atm::Var(v) => HashMap::from([(v.to_owned(), Type::Int)]),
        }
    }
}

#[cfg(test)]
mod typecheck_tests {
    use super::{typecheck, Atm, Exp, Program, Stmt, Tail, Type, Typecheck};
    use crate::c_var::{BinOp, UnaryOp};
    use std::collections::HashMap;

    #[test]
    fn check_var() {
        let result = Atm::Var("x".to_owned()).check();
        let expected = HashMap::from([("x".to_owned(), Type::Int)]);
        assert_eq!(result, expected)
    }

    #[test]
    fn check_int() {
        let result = Atm::Int(3).check();
        let expected = HashMap::new();
        assert_eq!(result, expected)
    }

    #[test]
    fn check_assign() {
        let result = Stmt::Assign {
            var: "x".to_owned(),
            exp: Exp::Read,
        }
        .check();
        let expected = HashMap::from([("x".to_owned(), Type::Int)]);
        assert_eq!(result, expected)
    }

    #[test]
    fn check_read() {
        let result = Exp::Read.check();
        let expected = HashMap::new();
        assert_eq!(result, expected)
    }

    #[test]
    fn check_unary() {
        let result = Exp::UnaryOp {
            exp: Atm::Var("x".to_owned()),
            op: UnaryOp::Neg,
        }
        .check();
        let expected = HashMap::from([("x".to_owned(), Type::Int)]);
        assert_eq!(result, expected)
    }

    #[test]
    fn check_binop() {
        let result = Exp::BinOp {
            exp1: Atm::Var("x".to_owned()),
            op: BinOp::Add,
            exp2: Atm::Var("y".to_owned()),
        }
        .check();
        let expected = HashMap::from([("x".to_owned(), Type::Int), ("y".to_owned(), Type::Int)]);
        assert_eq!(result, expected)
    }

    #[test]
    fn check_return() {
        let result = Tail::Return(Exp::Atm(Atm::Var("x".to_owned()))).check();
        let expected = HashMap::from([("x".to_owned(), Type::Int)]);
        assert_eq!(result, expected)
    }

    #[test]
    fn check_seq() {
        let result = Tail::Seq(
            Stmt::Assign {
                var: "x".to_owned(),
                exp: Exp::Read,
            },
            Box::new(Tail::Return(Exp::Atm(Atm::Var("x".to_owned())))),
        )
        .check();
        let expected = HashMap::from([("x".to_owned(), Type::Int)]);
        assert_eq!(result, expected)
    }

    #[test]
    fn check_prog() {
        let mut result = Program {
            blocks: HashMap::from([(
                "start".to_owned(),
                Tail::Return(Exp::Atm(Atm::Var("x".to_owned()))),
            )]),
            types: HashMap::new(),
        };
        typecheck(&mut result);
        let expected = Program {
            blocks: HashMap::from([(
                "start".to_owned(),
                Tail::Return(Exp::Atm(Atm::Var("x".to_owned()))),
            )]),
            types: HashMap::from([("x".to_owned(), Type::Int)]),
        };
        assert_eq!(result, expected)
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Int => f.write_str("int"),
        }
    }
}
