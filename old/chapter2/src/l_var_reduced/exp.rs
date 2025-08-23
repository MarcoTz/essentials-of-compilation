use super::{Atm, BinOp, UnaryOp, UsedVars, Var};
use std::{collections::HashSet, fmt};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Exp {
    Atm(Atm),
    Assign {
        name: Var,
        bound_term: Box<Exp>,
        in_term: Box<Exp>,
    },
    InputInt,
    UnaryOp {
        op: UnaryOp,
        exp: Atm,
    },
    BinOp {
        exp1: Atm,
        op: BinOp,
        exp2: Atm,
    },
}

impl From<Atm> for Exp {
    fn from(at: Atm) -> Exp {
        Exp::Atm(at)
    }
}

impl UsedVars for Exp {
    fn used_vars(&self) -> HashSet<Var> {
        match self {
            Exp::Atm(at) => at.used_vars(),
            Exp::Assign {
                name,
                bound_term,
                in_term,
            } => {
                let mut used = bound_term.used_vars();
                used.extend(in_term.used_vars());
                used.insert(name.clone());
                used
            }
            Exp::InputInt => HashSet::new(),
            Exp::UnaryOp { op: _, exp } => exp.used_vars(),
            Exp::BinOp { exp1, op: _, exp2 } => {
                let mut used = exp1.used_vars();
                used.extend(exp2.used_vars());
                used
            }
        }
    }
}

impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Exp::Atm(atm) => atm.fmt(f),
            Exp::Assign {
                name,
                bound_term,
                in_term,
            } => write!(f, "(let [{name} {bound_term}] {in_term})"),
            Exp::InputInt => f.write_str("read"),
            Exp::UnaryOp { op, exp } => write!(f, "({op} {exp})"),
            Exp::BinOp { exp1, op, exp2 } => write!(f, "({op} {exp1} {exp2})"),
        }
    }
}

#[cfg(test)]
mod exp_tests {
    use super::{BinOp, Exp, UnaryOp};

    #[test]
    fn display_input() {
        let result = format!("{}", Exp::InputInt);
        let expected = "read";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_unary() {
        let result = format!(
            "{}",
            Exp::UnaryOp {
                op: UnaryOp::Neg,
                exp: 1.into()
            }
        );
        let expected = "(- 1)";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_bin() {
        let result = format!(
            "{}",
            Exp::BinOp {
                op: BinOp::Add,
                exp1: 6.into(),
                exp2: 2.into()
            }
        );
        let expected = "(+ 6 2)";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_let() {
        let result = format!(
            "{}",
            Exp::Assign {
                name: "x".to_owned(),
                bound_term: Box::new(Exp::Atm(2.into())),
                in_term: Box::new(Exp::Atm("x".to_owned().into()))
            }
        );
        let expected = "(let [x 2] x)";
        assert_eq!(result, expected)
    }
}
