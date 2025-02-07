use super::{typecheck::Type, Label, Tail, Var};
use std::{collections::HashMap, fmt};

#[derive(Debug, PartialEq, Eq)]
pub struct Program {
    pub blocks: HashMap<Label, Tail>,
    pub types: HashMap<Var, Type>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let block_strs = self
            .blocks
            .iter()
            .map(|(label, tail)| format!("{label}: {tail}"))
            .collect::<Vec<String>>();
        write!(f, "{}", block_strs.join(", "))?;
        if !self.types.is_empty() {
            writeln!(f, "\n")?;
            for (var, ty) in self.types.iter() {
                writeln!(f, "{var}:{ty}")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod prog_test {
    use super::{Program, Tail};
    use crate::c_var::Exp;
    use std::collections::HashMap;

    #[test]
    fn display_empty() {
        let result = format!(
            "{}",
            Program {
                blocks: HashMap::new(),
                types: HashMap::new()
            }
        );
        let expected = "";
        assert_eq!(result, expected)
    }

    #[test]
    fn display_simple() {
        let result = format!(
            "{}",
            Program {
                blocks: HashMap::from([("main".to_owned(), Tail::Return(Exp::Read))]),
                types: HashMap::new()
            }
        );
        let expected = "main: return read;";
        assert_eq!(result, expected)
    }
}
