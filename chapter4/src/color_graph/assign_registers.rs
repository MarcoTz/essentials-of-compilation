use super::{Color, Coloring, RegisterAssignment};
use crate::errors::Error;
use chapter2::x86_var::Reg;
use std::collections::HashMap;

pub fn num_reg(color: Color) -> Result<Reg, Error> {
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

pub fn assign_registers(coloring: Coloring) -> Result<RegisterAssignment, Error> {
    let mut assignment = HashMap::new();
    for (var, color) in coloring.into_iter() {
        let reg = num_reg(color)?;
        assignment.insert(var, reg);
    }
    Ok(assignment)
}
