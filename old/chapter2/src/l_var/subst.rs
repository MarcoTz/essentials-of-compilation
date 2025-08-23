use super::syntax::{Exp, Program, Var};
use std::collections::HashMap;

pub trait Subst {
    fn subst_vars(self, vars: &HashMap<Var, Var>) -> Self;
}

impl Subst for Program {
    fn subst_vars(self, vars: &HashMap<Var, Var>) -> Self {
        Program {
            exp: self.exp.subst_vars(vars),
        }
    }
}

impl Subst for Exp {
    fn subst_vars(self, vars: &HashMap<Var, Var>) -> Self {
        match self {
            Exp::Name(v) => match vars.get(&v) {
                None => Exp::Name(v),
                Some(var) => Exp::Name(var.clone()),
            },
            Exp::Assign {
                name,
                bound_term,
                in_term,
            } => {
                let bound_repl = bound_term.subst_vars(vars);
                let in_repl = in_term.subst_vars(vars);
                let new_name = match vars.get(&name) {
                    None => name,
                    Some(new_name) => new_name.clone(),
                };
                Exp::Assign {
                    name: new_name,
                    bound_term: Box::new(bound_repl),
                    in_term: Box::new(in_repl),
                }
            }
            Exp::UnaryOp { op, exp } => Exp::UnaryOp {
                op,
                exp: Box::new(exp.subst_vars(vars)),
            },
            Exp::BinOp { exp1, op, exp2 } => Exp::BinOp {
                exp1: Box::new(exp1.subst_vars(vars)),
                op,
                exp2: Box::new(exp2.subst_vars(vars)),
            },
            _ => self,
        }
    }
}

#[cfg(test)]
mod subst_tests {
    use super::{Exp, HashMap, Subst, Var};

    fn example_vars() -> HashMap<Var, Var> {
        HashMap::from([
            ("x".to_owned(), "y".to_owned()),
            ("z".to_owned(), "a".to_owned()),
        ])
    }

    #[test]
    fn subst_read() {
        let result = Exp::InputInt.subst_vars(&example_vars());
        let expected = Exp::InputInt;
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_name() {
        let result = Exp::Name("x".to_owned()).subst_vars(&example_vars());
        let expected = Exp::Name("y".to_owned());
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_assign_same() {
        let result = Exp::Assign {
            name: "z".to_owned(),
            bound_term: Box::new(Exp::Name("x".to_owned())),
            in_term: Box::new(Exp::Name("z".to_owned())),
        }
        .subst_vars(&example_vars());
        let expected = Exp::Assign {
            name: "a".to_owned(),
            bound_term: Box::new(Exp::Name("y".to_owned())),
            in_term: Box::new(Exp::Name("a".to_owned())),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_assign_diff() {
        let result = Exp::Assign {
            name: "b".to_owned(),
            bound_term: Box::new(Exp::Name("x".to_owned())),
            in_term: Box::new(Exp::Name("b".to_owned())),
        }
        .subst_vars(&example_vars());
        let expected = Exp::Assign {
            name: "b".to_owned(),
            bound_term: Box::new(Exp::Name("y".to_owned())),
            in_term: Box::new(Exp::Name("b".to_owned())),
        };
        assert_eq!(result, expected)
    }
}
