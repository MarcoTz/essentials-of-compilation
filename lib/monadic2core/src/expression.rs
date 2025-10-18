use super::{BlockAccum, Error, ExplicateControl};

impl ExplicateControl for monadic::Expression {
    type Target = core::Expression;
    fn explicate_control(self, accum: &mut BlockAccum) -> Result<Self::Target, Error> {
        match self {
            monadic::Expression::Atm(atm) => {
                Ok(core::Expression::Atm(atm.explicate_control(accum)?))
            }
            monadic::Expression::ReadInt => Ok(core::Expression::ReadInt),
            monadic::Expression::UnaryOp { arg, op } => {
                let arg_exp = arg.explicate_control(accum)?;
                Ok(core::Expression::un(arg_exp, op))
            }
            monadic::Expression::BinaryOp { fst, op, snd } => {
                let fst_exp = fst.explicate_control(accum)?;
                let snd_exp = snd.explicate_control(accum)?;
                Ok(core::Expression::bin(fst_exp, op, snd_exp))
            }
            monadic::Expression::Cmp { left, cmp, right } => {
                let left_exp = left.explicate_control(accum)?;
                let right_exp = right.explicate_control(accum)?;
                Ok(core::Expression::cmp(left_exp, cmp, right_exp))
            }
        }
    }
}
