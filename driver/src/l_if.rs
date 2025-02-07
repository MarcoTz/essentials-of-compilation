use super::Driver;

pub struct LIfDriver {
    print_intermediary: bool,
}

impl Driver for LIfDriver {
    type Target = chapter4::x86_if::Program;

    fn is_debug(&self) -> bool {
        self.print_intermediary
    }

    fn compile(&self, _input: &str) -> Result<Self::Target, Box<dyn std::error::Error>> {
        todo!()
    }

    fn evaluate(&self, _prog: Self::Target) -> Result<String, Box<dyn std::error::Error>> {
        todo!()
    }
}
