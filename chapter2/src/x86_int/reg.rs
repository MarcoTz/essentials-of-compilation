use std::fmt;

pub enum Reg {
    Rsp,
    Rbp,
    Rax,
    Rbx,
    Rcx,
    Rdx,
    Rsi,
    Rdi,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Reg::Rsp => f.write_str("rsp"),
            Reg::Rbp => f.write_str("rbp"),
            Reg::Rax => f.write_str("rax"),
            Reg::Rbx => f.write_str("rbx"),
            Reg::Rcx => f.write_str("rcx"),
            Reg::Rdx => f.write_str("rdx"),
            Reg::Rsi => f.write_str("rsi"),
            Reg::Rdi => f.write_str("rdi"),
            Reg::R8 => f.write_str("r8"),
            Reg::R9 => f.write_str("r9"),
            Reg::R10 => f.write_str("r10"),
            Reg::R11 => f.write_str("r11"),
            Reg::R12 => f.write_str("r12"),
            Reg::R13 => f.write_str("r13"),
            Reg::R14 => f.write_str("r14"),
            Reg::R15 => f.write_str("r15"),
        }
    }
}

#[cfg(test)]
mod reg_tests {
    use super::Reg;

    #[test]
    fn display_rsp() {
        let result = format!("{}", Reg::Rsp);
        let expected = "rsp";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_rbp() {
        let result = format!("{}", Reg::Rbp);
        let expected = "rbp";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_rax() {
        let result = format!("{}", Reg::Rax);
        let expected = "rax";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_rbx() {
        let result = format!("{}", Reg::Rbx);
        let expected = "rbx";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_rcx() {
        let result = format!("{}", Reg::Rcx);
        let expected = "rcx";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_rdx() {
        let result = format!("{}", Reg::Rdx);
        let expected = "rdx";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_rsi() {
        let result = format!("{}", Reg::Rsi);
        let expected = "rsi";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_rdi() {
        let result = format!("{}", Reg::Rdi);
        let expected = "rdi";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_r8() {
        let result = format!("{}", Reg::R8);
        let expected = "r8";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_r9() {
        let result = format!("{}", Reg::R9);
        let expected = "r9";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_r10() {
        let result = format!("{}", Reg::R10);
        let expected = "r10";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_r11() {
        let result = format!("{}", Reg::R11);
        let expected = "r11";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_r12() {
        let result = format!("{}", Reg::R12);
        let expected = "r12";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_r13() {
        let result = format!("{}", Reg::R13);
        let expected = "r13";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_r14() {
        let result = format!("{}", Reg::R14);
        let expected = "r14";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_r15() {
        let result = format!("{}", Reg::R15);
        let expected = "r15";
        assert_eq!(result, expected)
    }
}
