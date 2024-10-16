use super::SelectInstructions;
use crate::{
    l_var_reduced::Stmt,
    x86_var::{Arg, Instr, Prog, Reg},
};

impl SelectInstructions for Stmt {
    type Target = Prog;
    fn select_instructions(self) -> Self::Target {
        match self {
            Stmt::Print(at) => {
                let atm_instr = at.select_instructions();
                let mut prog = vec![atm_instr];
                prog.push(Instr::MovQ(Arg::Reg(Reg::Rax), Arg::Reg(Reg::Rdi)));
                prog.push(Instr::CallQ("print_int".to_owned(), 0));
                prog
            }
            Stmt::Exp(exp) => exp.select_instructions(),
            Stmt::Assign { name, exp } => {
                let mut prog = exp.select_instructions();
                prog.push(Instr::MovQ(Arg::Reg(Reg::Rax), Arg::Var(name)));
                prog
            }
        }
    }
}

#[cfg(test)]
mod stmt_tests {
    use super::{Arg, Instr, Reg, SelectInstructions, Stmt};
    use crate::l_var_reduced::Exp;

    #[test]
    fn select_print() {
        let result = Stmt::Print(2.into()).select_instructions();
        let expected = vec![
            Instr::MovQ(Arg::Intermediate(2), Arg::Reg(Reg::Rax)),
            Instr::MovQ(Arg::Reg(Reg::Rax), Arg::Reg(Reg::Rdi)),
            Instr::CallQ("print_int".to_owned(), 0),
        ];
        assert_eq!(result, expected)
    }

    #[test]
    fn select_exp() {
        let result = Stmt::Exp(Exp::InputInt).select_instructions();
        let expected = vec![Instr::CallQ("read_int".to_owned(), 0)];
        assert_eq!(result, expected)
    }

    #[test]
    fn select_assign() {
        let result = Stmt::Assign {
            name: "x".to_owned(),
            exp: Exp::Atm(2.into()),
        }
        .select_instructions();
        let expected = vec![
            Instr::MovQ(Arg::Intermediate(2), Arg::Reg(Reg::Rax)),
            Instr::MovQ(Arg::Reg(Reg::Rax), Arg::Var("x".to_owned())),
        ];
        assert_eq!(result, expected)
    }
}
