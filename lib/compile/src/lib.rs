use parser::parse_program;
use std::{fs::read_to_string, path::PathBuf};
use syntax::lang::Program;
use uniquify::uniquify;

mod errors;

pub use errors::Error;

pub struct Compiler {
    debug: bool,
    source: String,
    parsed: Option<Program>,
    uniquified: Option<Program>,
}

impl Compiler {
    pub fn new(debug: bool, source: PathBuf) -> Result<Compiler, Error> {
        let source_contents = read_to_string(&source).map_err(|_| Error::ReadFile(source))?;
        Ok(Compiler {
            debug,
            source: source_contents,
            parsed: None,
            uniquified: None,
        })
    }

    pub fn parse(&mut self) -> Result<(), Error> {
        let parsed = parse_program(&self.source)?;
        if self.debug {
            println!("=== Parsed ===");
            println!("{parsed}");
            println!("");
        }
        self.parsed = Some(parsed);
        Ok(())
    }

    pub fn get_parsed(&mut self) -> Result<Program, Error> {
        if self.parsed.is_none() {
            self.parse()?;
        }
        Ok(self.parsed.as_ref().unwrap().clone())
    }

    pub fn uniquify(&mut self) -> Result<(), Error> {
        let uniquified = uniquify(self.get_parsed()?);
        if self.debug {
            println!("=== uniquified ===");
            println!("{uniquified}");
            println!();
        }
        self.uniquified = Some(uniquified);
        Ok(())
    }

    pub fn get_uniquified(&mut self) -> Result<Program, Error> {
        if self.uniquified.is_none() {
            self.uniquify()?;
        }
        Ok(self.uniquified.as_ref().unwrap().clone())
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
