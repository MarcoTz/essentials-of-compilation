use super::{Label, Tail};
use std::{collections::HashMap, fmt};

#[derive(Debug)]
pub struct Program {
    pub blocks: HashMap<Label, Tail>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let block_strs = self
            .blocks
            .iter()
            .map(|(label, tail)| format!("{label}: {tail}"))
            .collect::<Vec<String>>();
        write!(f, "{}", block_strs.join(", "))
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
                blocks: HashMap::from([("main".to_owned(), Tail::Return(Exp::Read))])
            }
        );
        let expected = "main: return read;";
        assert_eq!(result, expected)
    }
}
