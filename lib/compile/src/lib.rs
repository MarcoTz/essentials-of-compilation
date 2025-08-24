use assign_homes::assign_homes;
use explicate_control::explicate_control;
use parser::parse_program;
use patch_instructions::patch_instructions;
use remove_complex_operands::remove_complex_operands;
use select_instructions::select_instructions;
use std::{fs::read_to_string, path::PathBuf};
use syntax::{lang, lang_c, lang_mon, x86};
use uniquify::uniquify;

mod errors;

pub use errors::Error;

pub struct Compiler {
    debug: bool,
    source: String,
    parsed: Option<lang::Program>,
    uniquified: Option<lang::Program>,
    monadic: Option<lang_mon::Program>,
    explicated: Option<lang_c::Program>,
    selected: Option<x86::VarProg>,
    assigned: Option<x86::Prog>,
    patched: Option<x86::Prog>,
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
            explicated: None,
            selected: None,
            assigned: None,
            patched: None,
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

    pub fn explicate_control(&mut self) -> Result<(), Error> {
        let explicated = explicate_control(self.get_monadic()?);
        if self.debug {
            println!("=== Explicate Control ===");
            println!("{explicated}");
            println!();
        }
        self.explicated = Some(explicated);
        Ok(())
    }

    pub fn get_explicated(&mut self) -> Result<lang_c::Program, Error> {
        if self.explicated.is_none() {
            self.explicate_control()?;
        }
        Ok(self.explicated.as_ref().unwrap().clone())
    }

    pub fn select_instructions(&mut self) -> Result<(), Error> {
        let selected = select_instructions(self.get_explicated()?);
        if self.debug {
            println!("=== Select Instructions ===");
            println!("{selected}");
            println!();
        }
        self.selected = Some(selected);
        Ok(())
    }

    pub fn get_selected(&mut self) -> Result<x86::VarProg, Error> {
        if self.selected.is_none() {
            self.select_instructions()?;
        }
        Ok(self.selected.as_ref().unwrap().clone())
    }

    pub fn assign_homes(&mut self) -> Result<(), Error> {
        let assigned = assign_homes(self.get_selected()?);
        if self.debug {
            println!("=== Assign Homes ===");
            println!("{assigned}");
            println!();
        }
        self.assigned = Some(assigned);
        Ok(())
    }

    pub fn get_assigned(&mut self) -> Result<x86::Prog, Error> {
        if self.assigned.is_none() {
            self.assign_homes()?;
        }
        Ok(self.assigned.as_ref().unwrap().clone())
    }

    pub fn patch_instructions(&mut self) -> Result<(), Error> {
        let patched = patch_instructions(self.get_assigned()?);
        if self.debug {
            println!("=== Patch Instructions ===");
            println!("{patched}");
            println!();
        }
        self.patched = Some(patched);
        Ok(())
    }

    pub fn get_patched(&mut self) -> Result<x86::Prog, Error> {
        if self.patched.is_none() {
            self.patch_instructions()?;
        }
        Ok(self.patched.as_ref().unwrap().clone())
    }
}

//let finalized = prelude_and_conclusion(patched);*/
