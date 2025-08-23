use parser::parse_program;
use remove_complex_operands::remove_complex_operands;
use std::{fs::read_to_string, path::PathBuf};
use syntax::{lang, lang_mon};
use uniquify::uniquify;

mod errors;

pub use errors::Error;

pub struct Compiler {
    debug: bool,
    source: String,
    parsed: Option<lang::Program>,
    uniquified: Option<lang::Program>,
    monadic: Option<lang_mon::Program>,
}

impl Compiler {
    pub fn new(debug: bool, source: PathBuf) -> Result<Compiler, Error> {
        let source_contents = read_to_string(&source).map_err(|_| Error::ReadFile(source))?;
        Ok(Compiler {
            debug,
            source: source_contents,
            parsed: None,
            uniquified: None,
            monadic: None,
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

    pub fn get_parsed(&mut self) -> Result<lang::Program, Error> {
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

    pub fn get_uniquified(&mut self) -> Result<lang::Program, Error> {
        if self.uniquified.is_none() {
            self.uniquify()?;
        }
        Ok(self.uniquified.as_ref().unwrap().clone())
    }

    pub fn remove_complex_operands(&mut self) -> Result<(), Error> {
        let monadic = remove_complex_operands(self.get_uniquified()?);
        if self.debug {
            println!("=== Remove Complex Operands ===");
            println!("{monadic}");
            println!();
        }
        self.monadic = Some(monadic);
        Ok(())
    }

    pub fn get_monadic(&mut self) -> Result<lang_mon::Program, Error> {
        if self.monadic.is_none() {
            self.remove_complex_operands()?;
        }
        Ok(self.monadic.as_ref().unwrap().clone())
    }
}

//let monadic = remove_complex_operands(uniquified);
//let explicated = explicate_control(monadic);
//let selected = select_instructions(explicated);
//let assigned = assign_homes(selected);
//let patched = patch_instructions(assigned);
//let finalized = prelude_and_conclusion(patched);*/
