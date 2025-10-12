use std::collections::HashSet;

mod lang;
mod lang_mon;

pub trait UsedVars {
    fn used_vars(&self) -> HashSet<String>;
}

pub fn fresh_var(used_vars: &HashSet<String>) -> String {
    let mut num = 0;
    let mut var = format!("x{num}");
    while used_vars.contains(&var) {
        num += 1;
        var = format!("x{num}");
    }
    var
}
