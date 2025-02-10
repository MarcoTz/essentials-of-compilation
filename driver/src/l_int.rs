use super::Driver;
use chapter1::{eval::interp_lint, parser::parse_l_int};

pub struct LIntDriver;

impl Driver for LIntDriver {
    type Target = chapter1::syntax::Program;
    type Parsed = chapter1::syntax::Program;

    fn is_debug(&self) -> bool {
        false
    }

    fn parse(&self, input: &str) -> Result<Self::Parsed, Box<dyn std::error::Error>> {
        let mut input = input.to_owned();
        parse_l_int(&mut input).map_err(|err| Box::new(err) as Box<dyn std::error::Error>)
    }

    fn compile(&self, input: Self::Parsed) -> Result<Self::Target, Box<dyn std::error::Error>> {
        Ok(input)
    }

    fn evaluate(&self, prog: Self::Target) -> Result<String, Box<dyn std::error::Error>> {
        let res = interp_lint(prog);
        Ok(res.to_string())
    }
}
