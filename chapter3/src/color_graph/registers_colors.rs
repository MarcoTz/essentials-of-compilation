use super::{Color, Coloring, RegisterAssignment};
use crate::errors::Error;
use chapter2::x86_var::Reg;
use std::collections::HashMap;

pub fn reg_to_color(reg: &Reg) -> i32 {
    match reg {
        Reg::Rcx => 0,
        Reg::Rdx => 1,
        Reg::Rsi => 2,
        Reg::Rdi => 3,
        Reg::R8 => 4,
        Reg::R9 => 5,
        Reg::R10 => 6,
        Reg::Rbx => 7,
        Reg::R12 => 8,
        Reg::R13 => 9,
        Reg::R14 => 10,
        Reg::Rax => -1,
        Reg::Rsp => -2,
        Reg::Rbp => -3,
        Reg::R11 => -4,
        Reg::R15 => -5,
    }
}

pub fn color_to_reg(color: Color) -> Result<Reg, Error> {
    match color {
        0 => Ok(Reg::Rcx),
        1 => Ok(Reg::Rdx),
        2 => Ok(Reg::Rsi),
        3 => Ok(Reg::Rdi),
        4 => Ok(Reg::R8),
        5 => Ok(Reg::R9),
        6 => Ok(Reg::R10),
        7 => Ok(Reg::Rbx),
        8 => Ok(Reg::R12),
        9 => Ok(Reg::R13),
        10 => Ok(Reg::R14),
        -1 => Ok(Reg::Rax),
        -2 => Ok(Reg::Rsp),
        -3 => Ok(Reg::Rbp),
        -4 => Ok(Reg::R11),
        -5 => Ok(Reg::R15),
        _ => Err(Error::RegistersFull),
    }
}
