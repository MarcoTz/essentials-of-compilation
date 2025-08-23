use parser::parse_program;
use std::{fs::read_to_string, path::PathBuf};
use syntax::lang::Program;

mod errors;

pub use errors::Error;

pub struct Compiler {
    debug: bool,
    source: String,
}

impl Compiler {
    pub fn new(debug: bool, source: PathBuf) -> Result<Compiler, Error> {
        let source_contents = read_to_string(&source).map_err(|_| Error::ReadFile(source))?;
        Ok(Compiler {
            debug,
            source: source_contents,
        })
    }

    pub fn parse(&self) -> Result<Program, Error> {
        let parsed = parse_program(&self.source)?;
        if self.debug {
            println!("=== Parsed ===");
            println!("{parsed}");
            println!("");
        }
        Ok(parsed)
    }
}

//let parsed = parse_program(src)?;
/*let uniquified = parsed.uniquify();
let monadic = remove_complex_operands(uniquified);
let explicated = explicate_control(monadic);
let selected = select_instructions(explicated);
let assigned = assign_homes(selected);
let patched = patch_instructions(assigned);
let finalized = prelude_and_conclusion(patched);*/
