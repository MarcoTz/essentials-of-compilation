use super::{BlockAccum, Error, ExplicateControl};

impl ExplicateControl for monadic::Statement {
    type Target = ();
    fn explicate_control(self, state: &mut BlockAccum) -> Result<(), Error> {
        match self {
            monadic::Statement::Return(atm) => {
                let cont = core::Continuation::Return(atm.explicate_control(state)?);
                state.next_block(cont);
                Ok(())
            }
            monadic::Statement::Print(atm) => {
                let stmt = core::Statement::Print(atm.explicate_control(state)?);
                state.push_stmt(stmt);
                Ok(())
            }
            monadic::Statement::Assign { var, bound } => {
                let bound_exp = bound.explicate_control(state)?;
                state.push_stmt(core::Statement::assign(&var, bound_exp));
                Ok(())
            }
            monadic::Statement::Set { var, bound } => {
                let bound_exp = bound.explicate_control(state)?;
                state.push_stmt(core::Statement::set(&var, bound_exp));
                Ok(())
            }
            monadic::Statement::If {
                cond: cond_exp,
                then_block,
                else_block,
            } => {
                let cond = cond_exp.explicate_control(state)?;
                let then_label = state.fresh_label();
                let else_label = state.fresh_label();
                let cont = core::Continuation::If {
                    cond,
                    then_label: then_label.clone(),
                    else_label: else_label.clone(),
                };
                state.next_block(cont);
                state.current_label = then_label;
                then_block.explicate_control(state)?;
                state.current_label = else_label;
                else_block.explicate_control(state)?;
                Ok(())
            }
            monadic::Statement::While { cond, while_block } => {
                let cond = cond.explicate_control(state)?;
                let while_label = state.fresh_label();
                let next_label = state.fresh_label();
                let cont = core::Continuation::While {
                    cond,
                    while_label: while_label.clone(),
                    next_label: next_label.clone(),
                };
                state.next_block(cont);
                state.current_label = while_label;
                while_block.explicate_control(state)?;
                state.current_label = next_label;
                Ok(())
            }
        }
    }
}
