use std::{fs::read_to_string, path::PathBuf};

pub mod consts;
pub mod l_if;
pub mod l_int;
pub mod l_var;
pub mod l_var_reg;

pub trait Driver {
    type Target;

    fn compile(&self, source: &str) -> Result<Self::Target, Box<dyn std::error::Error>>;
    fn evaluate(&self, prog: Self::Target) -> Result<String, Box<dyn std::error::Error>>;
    fn is_debug(&self) -> bool;

    fn compile_file(&self, path: &PathBuf) -> Result<Self::Target, Box<dyn std::error::Error>> {
        let contents = read_to_string(path)?;
        self.compile(&contents)
    }

    fn compile_and_eval(&self, input: &str) -> Result<String, Box<dyn std::error::Error>> {
        let compiled = self.compile(input)?;
        self.evaluate(compiled)
    }

    fn compile_and_eval_file(&self, input: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
        let compiled = self.compile_file(input)?;
        self.evaluate(compiled)
    }

    fn debug(&self, msg: String) {
        if self.is_debug() {
            println!("{msg}");
        }
    }
}
