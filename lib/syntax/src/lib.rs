use std::collections::HashSet;
pub mod lang;
pub mod lang_mon;
pub mod x86;

mod ops;
pub use ops::{BinaryOperation, UnaryOperation};

pub fn fresh_var(used_vars: &mut HashSet<String>) -> String {
    let mut num = 0;
    let prefix = "x";
    while used_vars.contains(&format!("{prefix}{num}")) {
        num += 1;
    }
    let var = format!("{prefix}{num}");
    used_vars.insert(var.clone());
    var
}
