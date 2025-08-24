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
