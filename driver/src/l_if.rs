use super::Driver;

pub struct LIfDriver {}

impl Driver for LIfDriver {
    type Target = chapter4::x86_if::Program;

    fn compile(
        &self,
        _input: &str,
        _print_intermediary: bool,
    ) -> Result<Self::Target, Box<dyn std::error::Error>> {
        todo!()
    }

    fn evaluate(&self, _prog: Self::Target) -> Result<String, Box<dyn std::error::Error>> {
        todo!()
    }
}
