pub mod consts;
pub mod l_if;
pub mod l_int;
pub mod l_var;
pub mod l_var_reg;

pub trait Driver {
    type Parsed;
    type Target;

    fn parse(&self, source: &str) -> Result<Self::Parsed, Box<dyn std::error::Error>>;
    fn compile(&self, source: Self::Parsed) -> Result<Self::Target, Box<dyn std::error::Error>>;
    fn evaluate(&self, prog: Self::Target) -> Result<String, Box<dyn std::error::Error>>;
    fn is_debug(&self) -> bool;

    fn debug(&self, msg: &str) {
        if self.is_debug() {
            println!("{msg}\n");
        }
    }
}
