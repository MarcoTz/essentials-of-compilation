use super::{RemoveComplexOperands, exp_to_atm};
use std::collections::HashSet;

impl RemoveComplexOperands for surface::Expression {
    type Target = (Vec<monadic::Statement>, monadic::Expression);

    fn remove_complex_operands(self, used_vars: &mut HashSet<String>) -> Self::Target {
        match self {
            surface::Expression::Literal(i) => (vec![], monadic::Atom::Integer(i).into()),
            surface::Expression::Bool(b) => (vec![], monadic::Atom::Bool(b).into()),
            surface::Expression::Variable(v) => (vec![], monadic::Atom::Variable(v).into()),
            surface::Expression::ReadInt => (vec![], monadic::Expression::ReadInt),

            surface::Expression::BinOp { fst, op, snd } => {
                let (fst_exps, fst_last) = fst.remove_complex_operands(used_vars);
                let (snd_exps, snd_last) = snd.remove_complex_operands(used_vars);
                let mut exps = vec![];
                exps.extend(fst_exps);
                let fst_atm = if let monadic::Expression::Atm(atm) = fst_last {
                    atm
                } else {
                    let (assignment, atm) = exp_to_atm(fst_last, used_vars, false);
                    exps.push(assignment);
                    atm
                };
                exps.extend(snd_exps);
                let snd_atm = if let monadic::Expression::Atm(atm) = snd_last {
                    atm
                } else {
                    let (assignment, atm) = exp_to_atm(snd_last, used_vars, false);
                    exps.push(assignment);
                    atm
                };
                (exps, monadic::Expression::bin(fst_atm, op, snd_atm))
            }
            surface::Expression::UnOp { arg, op } => {
                let (mut exps, last) = arg.remove_complex_operands(used_vars);
                if let monadic::Expression::Atm(atm) = last {
                    (exps, monadic::Expression::un(atm, op))
                } else {
                    let (assignment, atm) = exp_to_atm(last, used_vars, false);
                    exps.push(assignment);
                    (exps, monadic::Expression::un(atm, op))
                }
            }
            surface::Expression::Cmp { left, cmp, right } => {
                let (left_exps, left_last) = left.remove_complex_operands(used_vars);
                let (right_exps, right_last) = right.remove_complex_operands(used_vars);
                let mut exps = left_exps;
                let left_atm = if let monadic::Expression::Atm(atm) = left_last {
                    atm
                } else {
                    let (assignment, atm) = exp_to_atm(left_last, used_vars, false);
                    exps.push(assignment);
                    atm
                };
                exps.extend(right_exps);
                let right_atm = if let monadic::Expression::Atm(atm) = right_last {
                    atm
                } else {
                    let (assignment, atm) = exp_to_atm(right_last, used_vars, false);
                    exps.push(assignment);
                    atm
                };
                (exps, monadic::Expression::cmp(left_atm, cmp, right_atm))
            }
        }
    }
}
