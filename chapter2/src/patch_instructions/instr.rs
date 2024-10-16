use super::PatchInstructions;
use crate::x86_int::{Arg, Instr, Reg};

impl PatchInstructions for Instr<Arg> {
    type Target = Vec<Instr<Arg>>;
    fn patch(self) -> Self::Target {
        let max_intermediate = (2 as i64).pow(16);
        match self {
            Instr::AddQ(Arg::Deref(Reg::Rbp, offset1), Arg::Deref(Reg::Rbp, offset2)) => {
                vec![
                    Instr::MovQ(Arg::Deref(Reg::Rbp, offset1), Arg::Reg(Reg::Rax)),
                    Instr::AddQ(Arg::Reg(Reg::Rax), Arg::Deref(Reg::Rbp, offset2)),
                ]
            }
            Instr::SubQ(Arg::Deref(Reg::Rbp, offset1), Arg::Deref(Reg::Rbp, offset2)) => {
                vec![
                    Instr::MovQ(Arg::Deref(Reg::Rbp, offset1), Arg::Reg(Reg::Rax)),
                    Instr::SubQ(Arg::Reg(Reg::Rax), Arg::Deref(Reg::Rbp, offset2)),
                ]
            }
            Instr::MovQ(Arg::Deref(Reg::Rbp, offset1), Arg::Deref(Reg::Rbp, offset2)) => {
                vec![
                    Instr::MovQ(Arg::Deref(Reg::Rbp, offset1), Arg::Reg(Reg::Rax)),
                    Instr::MovQ(Arg::Reg(Reg::Rax), Arg::Deref(Reg::Rbp, offset2)),
                ]
            }

            Instr::AddQ(Arg::Deref(Reg::Rbp, offset), Arg::Intermediate(val)) => {
                if val > max_intermediate {
                    vec![
                        Instr::MovQ(Arg::Intermediate(val), Arg::Reg(Reg::Rax)),
                        Instr::AddQ(Arg::Deref(Reg::Rbp, offset), Arg::Reg(Reg::Rax)),
                    ]
                } else {
                    vec![Instr::AddQ(
                        Arg::Deref(Reg::Rbp, offset),
                        Arg::Intermediate(val),
                    )]
                }
            }
            Instr::AddQ(Arg::Intermediate(val), Arg::Deref(Reg::Rbp, offset)) => {
                if val > max_intermediate {
                    vec![
                        Instr::MovQ(Arg::Intermediate(val), Arg::Reg(Reg::Rax)),
                        Instr::AddQ(Arg::Reg(Reg::Rax), Arg::Deref(Reg::Rbp, offset)),
                    ]
                } else {
                    vec![Instr::AddQ(
                        Arg::Deref(Reg::Rbp, offset),
                        Arg::Intermediate(val),
                    )]
                }
            }
            Instr::SubQ(Arg::Deref(Reg::Rbp, offset), Arg::Intermediate(val)) => {
                if val > max_intermediate {
                    vec![
                        Instr::MovQ(Arg::Intermediate(val), Arg::Reg(Reg::Rax)),
                        Instr::SubQ(Arg::Deref(Reg::Rbp, offset), Arg::Reg(Reg::Rax)),
                    ]
                } else {
                    vec![Instr::SubQ(
                        Arg::Deref(Reg::Rbp, offset),
                        Arg::Intermediate(val),
                    )]
                }
            }
            Instr::SubQ(Arg::Intermediate(val), Arg::Deref(Reg::Rbp, offset)) => {
                if val > max_intermediate {
                    vec![
                        Instr::MovQ(Arg::Intermediate(val), Arg::Reg(Reg::Rax)),
                        Instr::SubQ(Arg::Reg(Reg::Rax), Arg::Deref(Reg::Rbp, offset)),
                    ]
                } else {
                    vec![Instr::SubQ(
                        Arg::Deref(Reg::Rbp, offset),
                        Arg::Intermediate(val),
                    )]
                }
            }

            Instr::MovQ(Arg::Deref(Reg::Rbp, offset), Arg::Intermediate(val)) => {
                if val > max_intermediate {
                    vec![
                        Instr::MovQ(Arg::Intermediate(val), Arg::Reg(Reg::Rax)),
                        Instr::MovQ(Arg::Deref(Reg::Rbp, offset), Arg::Reg(Reg::Rax)),
                    ]
                } else {
                    vec![Instr::MovQ(
                        Arg::Deref(Reg::Rbp, offset),
                        Arg::Intermediate(val),
                    )]
                }
            }
            Instr::MovQ(Arg::Intermediate(val), Arg::Deref(Reg::Rbp, offset)) => {
                if val > max_intermediate {
                    vec![
                        Instr::MovQ(Arg::Intermediate(val), Arg::Reg(Reg::Rax)),
                        Instr::MovQ(Arg::Reg(Reg::Rax), Arg::Deref(Reg::Rbp, offset)),
                    ]
                } else {
                    vec![Instr::MovQ(
                        Arg::Deref(Reg::Rbp, offset),
                        Arg::Intermediate(val),
                    )]
                }
            }

            Instr::AddQ(a1, a2) => vec![Instr::AddQ(a1, a2)],
            Instr::SubQ(a1, a2) => vec![Instr::SubQ(a1, a2)],
            Instr::MovQ(a1, a2) => vec![Instr::MovQ(a1, a2)],

            Instr::NegQ(a) => vec![Instr::NegQ(a)],
            Instr::PushQ(a) => vec![Instr::PushQ(a)],
            Instr::PopQ(a) => vec![Instr::PopQ(a)],

            Instr::CallQ(name, offset) => vec![Instr::CallQ(name, offset)],
            Instr::RetQ => vec![Instr::RetQ],
            Instr::Jump(l) => vec![Instr::Jump(l)],
        }
    }
}

#[cfg(test)]
mod instr_tests {
    use super::{Arg, Instr, PatchInstructions, Reg};

    #[test]
    fn add_reg_reg() {
        let result = Instr::AddQ(Arg::Reg(Reg::Rax), Arg::Reg(Reg::Rbx)).patch();
        let expected = vec![Instr::AddQ(Arg::Reg(Reg::Rax), Arg::Reg(Reg::Rbx))];
        assert_eq!(result, expected)
    }

    #[test]
    fn add_stack_stack() {
        let result = Instr::AddQ(Arg::Deref(Reg::Rbp, -8), Arg::Deref(Reg::Rbp, -16)).patch();
        let expected = vec![
            Instr::MovQ(Arg::Deref(Reg::Rbp, -8), Arg::Reg(Reg::Rax)),
            Instr::AddQ(Arg::Reg(Reg::Rax), Arg::Deref(Reg::Rbp, -16)),
        ];
        assert_eq!(result, expected)
    }

    #[test]
    fn add_stack_intermediate() {
        let result = Instr::AddQ(
            Arg::Intermediate((2 as i64).pow(18)),
            Arg::Deref(Reg::Rbp, -8),
        )
        .patch();
        let expected = vec![
            Instr::MovQ(Arg::Intermediate((2 as i64).pow(18)), Arg::Reg(Reg::Rax)),
            Instr::AddQ(Arg::Reg(Reg::Rax), Arg::Deref(Reg::Rbp, -8)),
        ];
        assert_eq!(result, expected)
    }
}
