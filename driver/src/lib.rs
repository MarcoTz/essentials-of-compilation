use std::{fs::read_to_string, path::PathBuf};

pub mod consts;
pub mod l_if;
pub mod l_int;
pub mod l_var;
pub mod l_var_reg;

pub trait Driver {
    type Target;

    fn compile(
        &self,
        source: &str,
        print_intermediary: bool,
    ) -> Result<Self::Target, Box<dyn std::error::Error>>;
    fn evaluate(&self, prog: Self::Target) -> Result<String, Box<dyn std::error::Error>>;

    fn compile_file(
        &self,
        path: &PathBuf,
        print_intermediary: bool,
    ) -> Result<Self::Target, Box<dyn std::error::Error>> {
        let contents = read_to_string(path)?;
        self.compile(&contents, print_intermediary)
    }

    fn compile_and_eval(
        &self,
        input: &str,
        print_intermediary: bool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let compiled = self.compile(input, print_intermediary)?;
        self.evaluate(compiled)
    }

    fn compile_and_eval_file(
        &self,
        input: &PathBuf,
        print_intermediary: bool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let compiled = self.compile_file(input, print_intermediary)?;
        self.evaluate(compiled)
    }
}
