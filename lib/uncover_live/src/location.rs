use syntax::x86::Reg;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Location {
    Variable(String),
    Register(Reg),
    Stack(i64),
}

impl From<Reg> for Location {
    fn from(reg: Reg) -> Location {
        Location::Register(reg)
    }
}

impl From<&str> for Location {
    fn from(var: &str) -> Location {
        Location::Variable(var.to_owned())
    }
}
