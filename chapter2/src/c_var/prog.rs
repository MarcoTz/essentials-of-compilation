use super::{Label, Tail};
use std::{collections::HashMap, fmt};

#[derive(Debug)]
pub struct Prog {
    pub blocks: HashMap<Label, Tail>,
}

impl fmt::Display for Prog {
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
    use super::{Prog, Tail};
    use crate::c_var::Exp;
    use std::collections::HashMap;

    #[test]
    fn display_empty() {
        let result = format!(
            "{}",
            Prog {
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
            Prog {
                blocks: HashMap::from([("main".to_owned(), Tail::Return(Exp::Read))])
            }
        );
        let expected = "main: return read;";
        assert_eq!(result, expected)
    }
}
