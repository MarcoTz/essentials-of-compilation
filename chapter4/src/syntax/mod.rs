use std::fmt;
pub mod ops;
pub mod types;

pub use ops::{Cmp, Op};

pub type Var = String;

#[derive(Debug)]
pub enum Exp {
    Int(i64),
    Bool(bool),
    Var(Var),
    Prim {
        op: Op,
        args: Vec<Exp>,
    },
    Let {
        var: Var,
        bound_exp: Box<Exp>,
        in_exp: Box<Exp>,
    },
    If {
        ifc: Box<Exp>,
        thenc: Box<Exp>,
        elsec: Box<Exp>,
    },
}

impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Exp::Int(i) => f.write_str(&i.to_string()),
            Exp::Bool(b) => f.write_str(&b.to_string()),
            Exp::Var(v) => f.write_str(v),
            Exp::Prim { op, args } => write!(
                f,
                "(PrimOp {op} ({}))",
                args.iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Exp::Let {
                var,
                bound_exp,
                in_exp,
            } => write!(f, "(let ({var} {bound_exp}) ({in_exp}))"),
            Exp::If { ifc, thenc, elsec } => write!(f, "(If ({ifc}) ({thenc}) ({elsec})"),
        }
    }
}
