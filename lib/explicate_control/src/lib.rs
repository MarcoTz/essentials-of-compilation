use syntax::{lang_c, lang_mon};

pub fn explicate_control(prog: lang_mon::Program) -> lang_c::Program {
    let mut prog_c = lang_c::Program::new();
    prog_c.add_block("start", explicate_exp(prog.exp));
    prog_c
}

pub fn explicate_exp(exp: lang_mon::Expression) -> lang_c::Tail {
    match exp {
        lang_mon::Expression::Atm(atm) => mon_to_c_atm(atm).into(),
        lang_mon::Expression::InputInt => lang_c::Expression::InputInt.into(),
        lang_mon::Expression::UnaryOp { arg, op } => {
            lang_c::Expression::un(mon_to_c_atm(arg), op).into()
        }
        lang_mon::Expression::BinaryOp { fst, op, snd } => {
            lang_c::Expression::bin(mon_to_c_atm(fst), op, mon_to_c_atm(snd)).into()
        }
        lang_mon::Expression::LetIn {
            var,
            bound_exp,
            in_exp,
        } => {
            let bound_explicated = explicate_exp(*bound_exp);
            let assign = lang_c::Statement::assign(&var, bound_explicated.ret);
            let mut new_statements = bound_explicated.stmts;
            new_statements.push(assign);
            let in_explicated = explicate_exp(*in_exp);
            new_statements.extend(in_explicated.stmts);
            lang_c::Tail::new(in_explicated.ret, new_statements)
        }
    }
}

pub fn mon_to_c_atm(atm: lang_mon::Atom) -> lang_c::Atom {
    match atm {
        lang_mon::Atom::Integer(i) => lang_c::Atom::Integer(i),
        lang_mon::Atom::Variable(v) => lang_c::Atom::Variable(v),
    }
}

#[cfg(test)]
mod explicate_control_tests {
    use super::explicate_control;
    use syntax::{BinaryOperation, lang_c, lang_mon};

    #[test]
    fn explicate_let() {
        let result = explicate_control(lang_mon::Program::new(lang_mon::Expression::let_in(
            "y",
            lang_mon::Expression::let_in(
                "x0",
                lang_mon::Atom::Integer(20).into(),
                lang_mon::Expression::let_in(
                    "x1",
                    lang_mon::Atom::Integer(22).into(),
                    lang_mon::Expression::bin(
                        lang_mon::Atom::Variable("x0".to_owned()).into(),
                        BinaryOperation::Add,
                        lang_mon::Atom::Variable("x1".to_owned()).into(),
                    ),
                ),
            ),
            lang_mon::Atom::Variable("y".to_owned()).into(),
        )));
        let mut expected = lang_c::Program::new();
        expected.add_block(
            "start",
            lang_c::Tail {
                stmts: vec![
                    lang_c::Statement::Assign {
                        var: "x0".to_owned(),
                        bound: lang_c::Atom::Integer(20).into(),
                    },
                    lang_c::Statement::Assign {
                        var: "x1".to_owned(),
                        bound: lang_c::Atom::Integer(22).into(),
                    },
                    lang_c::Statement::Assign {
                        var: "y".to_owned(),
                        bound: lang_c::Expression::bin(
                            lang_c::Atom::Variable("x0".to_owned()),
                            BinaryOperation::Add,
                            lang_c::Atom::Variable("x1".to_owned()),
                        ),
                    },
                ],
                ret: lang_c::Atom::Variable("y".to_owned()).into(),
            },
        );
        assert_eq!(result, expected)
    }
}
