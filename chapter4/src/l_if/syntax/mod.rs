use crate::Var;
use std::fmt;
pub mod ops;
pub mod types;

pub use ops::{Cmp, Op};

pub type Program = Exp;

#[derive(Debug, Clone)]
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

impl Exp {
    pub fn subst(self, v: &Var, exp: Exp) -> Exp {
        match self {
            Exp::Int(_) | Exp::Bool(_) => exp,
            Exp::Var(var) => {
                if var == *v {
                    exp
                } else {
                    Exp::Var(var)
                }
            }
            Exp::Prim { op, args } => Exp::Prim {
                op,
                args: args
                    .into_iter()
                    .map(|arg| arg.subst(v, exp.clone()))
                    .collect(),
            },
            Exp::Let {
                var,
                bound_exp,
                in_exp,
            } => {
                if var == *v {
                    Exp::Let {
                        var,
                        bound_exp,
                        in_exp,
                    }
                } else {
                    Exp::Let {
                        var,
                        bound_exp: Box::new(bound_exp.subst(v, exp.clone())),
                        in_exp: Box::new(in_exp.subst(v, exp)),
                    }
                }
            }
            Exp::If { ifc, thenc, elsec } => Exp::If {
                ifc: Box::new(ifc.subst(v, exp.clone())),
                thenc: Box::new(thenc.subst(v, exp.clone())),
                elsec: Box::new(elsec.subst(v, exp)),
            },
        }
    }
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
