use super::Driver;

pub struct LIfDriver {
    print_intermediary: bool,
}

impl Driver for LIfDriver {
    type Target = chapter4::x86_if::Program;
    type Parsed = chapter4::l_if::syntax::Program;

    fn is_debug(&self) -> bool {
        self.print_intermediary
    }

    fn parse(&self, _input: &str) -> Result<Self::Parsed, Box<dyn std::error::Error>> {
        todo!()
    }

    fn compile(
        &self,
        _input: Self::Parsed,
        _prog_name: String,
    ) -> Result<Self::Target, Box<dyn std::error::Error>> {
        todo!()
    }

    fn evaluate(&self, _prog: Self::Target) -> Result<String, Box<dyn std::error::Error>> {
        todo!()
    }
}
