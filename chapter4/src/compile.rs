use crate::{
    errors::Error,
    l_if::{check::check, parse::parse},
    x86_if::Program,
};

pub fn compile(input: String) -> Result<Program, Error> {
    let parsed = parse(input)?;
    let _ = check(&parsed, &mut Default::default())?;
    todo!()
}
