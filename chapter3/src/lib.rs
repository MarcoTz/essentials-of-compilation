pub mod assign_homes;
pub mod color_graph;
pub mod compile;
pub mod errors;
pub mod interference_graph;
pub mod patch_instructions;
pub mod prelude_conclusion;
pub mod uncover_live;

pub type Var = String;

#[cfg(test)]
pub mod test_examples {
    use chapter2::x86_var::{Arg, Instr, Program, Reg};
    use std::collections::HashMap;

    pub fn example_prog1() -> Program {
        Program {
            blocks: HashMap::from([(
                "main".to_owned(),
                vec![
                    Instr::MovQ(Arg::Immediate(5), Arg::Var("a".to_owned())),
                    Instr::MovQ(Arg::Immediate(30), Arg::Var("b".to_owned())),
                    Instr::MovQ(Arg::Var("a".to_owned()), Arg::Var("c".to_owned())),
                    Instr::MovQ(Arg::Immediate(10), Arg::Var("b".to_owned())),
                    Instr::AddQ(Arg::Var("b".to_owned()), Arg::Var("c".to_owned())),
                ],
            )]),
            types: HashMap::new(),
        }
    }

    pub fn example_prog2() -> Program {
        Program {
            blocks: HashMap::from([(
                "main".to_owned(),
                vec![
                    Instr::MovQ(Arg::Immediate(1), Arg::Var("v".to_owned())),
                    Instr::MovQ(Arg::Immediate(42), Arg::Var("w".to_owned())),
                    Instr::MovQ(Arg::Var("v".to_owned()), Arg::Var("x".to_owned())),
                    Instr::AddQ(Arg::Immediate(7), Arg::Var("x".to_owned())),
                    Instr::MovQ(Arg::Var("x".to_owned()), Arg::Var("y".to_owned())),
                    Instr::MovQ(Arg::Var("x".to_owned()), Arg::Var("z".to_owned())),
                    Instr::AddQ(Arg::Var("w".to_owned()), Arg::Var("z".to_owned())),
                    Instr::MovQ(Arg::Var("y".to_owned()), Arg::Var("t".to_owned())),
                    Instr::NegQ(Arg::Var("t".to_owned())),
                    Instr::MovQ(Arg::Var("z".to_owned()), Arg::Reg(Reg::Rax)),
                    Instr::AddQ(Arg::Var("t".to_owned()), Arg::Reg(Reg::Rax)),
                    Instr::Jump("conclusion".to_owned()),
                ],
            )]),
            types: HashMap::new(),
        }
    }
}
