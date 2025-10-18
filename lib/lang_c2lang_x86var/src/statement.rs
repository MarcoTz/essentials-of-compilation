use super::SelectInstructions;
use definitions::PRINT_CALL;

impl SelectInstructions for core::Statement {
    type Target = Vec<lang_x86::Instruction<lang_x86::VarArg>>;
    type Arg = ();
    fn select_instructions(self, _: Self::Arg) -> Self::Target {
        match self {
            core::Statement::Assign { var, bound } => {
                bound.select_instructions(lang_x86::VarArg::Var(var))
            }
            core::Statement::Print(atm) => {
                let arg_loc = atm.select_instructions(());
                let mov = lang_x86::Instruction::MovQ {
                    src: arg_loc,
                    dest: lang_x86::Reg::Rdi.into(),
                };
                let print = lang_x86::Instruction::CallQ {
                    label: PRINT_CALL.to_owned(),
                };
                vec![mov, print]
            }
        }
    }
}
