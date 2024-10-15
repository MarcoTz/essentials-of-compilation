use super::Exp;
use std::fmt;

#[derive(Debug)]
pub enum ExpFunction {
    InputInt,
}

#[derive(Debug)]
pub enum StmtFunction {
    Print,
}
pub struct Call<T> {
    pub name: T,
    pub args: Vec<Exp>,
}

impl<T: fmt::Display> fmt::Display for Call<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}({})",
            self.name,
            self.args
                .iter()
                .map(|e| format!("{}", e))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl fmt::Display for ExpFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExpFunction::InputInt => f.write_str("input_int"),
        }
    }
}

impl fmt::Display for StmtFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StmtFunction::Print => f.write_str("print"),
        }
    }
}
