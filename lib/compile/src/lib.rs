use assign_homes::{
    assign_homes,
    color_graph::{Coloring, color_graph},
    interference_graph::{InterferenceGraph, build_graph},
    uncover_live::{AnnotProg, uncover_live},
};
use explicate_control::explicate_control;
use generate_prelude_conclusion::generate_prelude_conclusion;
use parser::parse_program;
use patch_instructions::patch_instructions;
use remove_complex_operands::remove_complex_operands;
use select_instructions::select_instructions;
use std::{
    fs::{File, create_dir_all, read_to_string},
    io::Write,
    path::PathBuf,
    process::Command,
};
use syntax::{lang, lang_c, lang_mon, x86};
use uniquify::uniquify;

mod errors;
mod paths;

pub use errors::Error;
use paths::{C_RUNTIME, get_asm_out, get_exe_out, get_object_out, get_runtime_object_out};

pub struct Compiler {
    debug: bool,
    source: String,
    parsed: Option<lang::Program>,
    uniquified: Option<lang::Program>,
    monadic: Option<lang_mon::Program>,
    explicated: Option<lang_c::Program>,
    selected: Option<x86::VarProgram>,
    uncovered: Option<AnnotProg>,
    interference_graph: Option<InterferenceGraph>,
    coloring: Option<Coloring>,
    assigned: Option<x86::Program>,
    patched: Option<x86::Program>,
    finalized: Option<x86::Program>,
    asm_out: PathBuf,
    object_out: PathBuf,
    pub exe_out: PathBuf,
}

impl Compiler {
    pub fn new(
        debug: bool,
        source: PathBuf,
        asm_out: Option<PathBuf>,
        object_out: Option<PathBuf>,
        exe_out: Option<PathBuf>,
    ) -> Result<Compiler, Error> {
        let prog_name = source
            .file_stem()
            .ok_or(Error::GetFileName(source.clone()))?;
        let asm_out = get_asm_out(asm_out, prog_name)?;
        let object_out = get_object_out(object_out, prog_name)?;
        let exe_out = get_exe_out(exe_out, prog_name)?;
        let source_contents = read_to_string(&source).map_err(|_| Error::ReadFile(source))?;

        Ok(Compiler {
            debug,
            source: source_contents,
            parsed: None,
            uniquified: None,
            monadic: None,
            explicated: None,
            selected: None,
            uncovered: None,
            interference_graph: None,
            coloring: None,
            assigned: None,
            patched: None,
            finalized: None,
            asm_out,
            object_out,
            exe_out,
        })
    }

    pub fn parse(&mut self) -> Result<(), Error> {
        let parsed = parse_program(&self.source)?;
        if self.debug {
            println!("=== Parsed ===");
            println!("{parsed}");
            println!();
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

    pub fn get_selected(&mut self) -> Result<x86::VarProgram, Error> {
        if self.selected.is_none() {
            self.select_instructions()?;
        }
        Ok(self.selected.as_ref().unwrap().clone())
    }

    pub fn uncover_live(&mut self) -> Result<(), Error> {
        let uncovered = uncover_live(self.get_selected()?);
        if self.debug {
            println!("=== Uncover Live ===");
            println!("{uncovered}");
            println!();
        }
        self.uncovered = Some(uncovered);
        Ok(())
    }

    pub fn get_uncovered(&mut self) -> Result<AnnotProg, Error> {
        if self.uncovered.is_none() {
            self.uncover_live()?;
        }
        Ok(self.uncovered.as_ref().unwrap().clone())
    }

    pub fn build_graph(&mut self) -> Result<(), Error> {
        let built = build_graph(&self.get_uncovered()?);
        if self.debug {
            println!("=== Interference Graph ===");
            println!("{built}");
            println!();
        }
        self.interference_graph = Some(built);
        Ok(())
    }

    pub fn get_graph(&mut self) -> Result<InterferenceGraph, Error> {
        if self.interference_graph.is_none() {
            self.build_graph()?;
        }
        Ok(self.interference_graph.as_ref().unwrap().clone())
    }

    pub fn color_graph(&mut self) -> Result<(), Error> {
        let coloring = color_graph(&self.get_graph()?);
        self.coloring = Some(coloring);
        Ok(())
    }

    pub fn get_coloring(&mut self) -> Result<Coloring, Error> {
        if self.coloring.is_none() {
            self.color_graph()?;
        }
        Ok(self.coloring.as_ref().unwrap().clone())
    }

    pub fn assign_homes(&mut self) -> Result<(), Error> {
        let assigned = assign_homes(self.get_selected()?, self.get_coloring()?);
        if self.debug {
            println!("=== Assign Homes ===");
            println!("{assigned}");
            println!();
        }
        self.assigned = Some(assigned);
        Ok(())
    }

    pub fn get_assigned(&mut self) -> Result<x86::Program, Error> {
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

    pub fn get_patched(&mut self) -> Result<x86::Program, Error> {
        if self.patched.is_none() {
            self.patch_instructions()?;
        }
        Ok(self.patched.as_ref().unwrap().clone())
    }

    pub fn generate_prelude_conclusion(&mut self) -> Result<(), Error> {
        let finalized = generate_prelude_conclusion(self.get_patched()?);
        if self.debug {
            println!("=== Generate Prelude and Conclusion ===");
            println!("{finalized}");
            println!();
        }
        self.finalized = Some(finalized);
        Ok(())
    }

    pub fn get_finalized(&mut self) -> Result<x86::Program, Error> {
        if self.finalized.is_none() {
            self.generate_prelude_conclusion()?;
        }
        Ok(self.finalized.as_ref().unwrap().clone())
    }

    pub fn write_asm(&mut self) -> Result<(), Error> {
        let finalized = self.get_finalized()?;
        let out_dir = self
            .asm_out
            .parent()
            .ok_or(Error::ParentNotFound(self.asm_out.clone()))?;
        create_dir_all(out_dir).map_err(|_| Error::CreateDir(out_dir.to_path_buf()))?;
        let mut out_file =
            File::create(&self.asm_out).map_err(|_| Error::CreateFile(self.asm_out.clone()))?;
        out_file
            .write_all(finalized.to_string().as_bytes())
            .map_err(|_| Error::WriteFile(self.asm_out.clone()))?;
        Ok(())
    }

    pub fn assemble(&mut self) -> Result<(), Error> {
        if !self.asm_out.exists() {
            self.write_asm()?;
        }
        let out_dir = self
            .object_out
            .parent()
            .ok_or(Error::ParentNotFound(self.object_out.clone()))?;
        create_dir_all(out_dir).map_err(|_| Error::CreateDir(out_dir.to_path_buf()))?;
        let mut gcc_cmd = Command::new("gcc");
        gcc_cmd
            .arg("-c")
            .arg("-x")
            .arg("assembler")
            .arg(&self.asm_out)
            .arg("-o")
            .arg(&self.object_out);
        let res = gcc_cmd
            .status()
            .map_err(|_| Error::RunCommand("gcc -c".to_owned()))?;
        if !res.success() {
            return Err(Error::RunCommand("gcc -c".to_owned()));
        }
        Ok(())
    }

    pub fn assemble_runtime(&self) -> Result<(), Error> {
        let runtime_in = PathBuf::from(C_RUNTIME);
        let runtime_out = get_runtime_object_out();
        if runtime_out.exists() {
            return Ok(());
        }

        let mut gcc_cmd = Command::new("gcc");
        gcc_cmd
            .arg("-c")
            .arg(&runtime_in)
            .arg("-o")
            .arg(&runtime_out);
        let res = gcc_cmd
            .status()
            .map_err(|_| Error::RunCommand("gcc -c".to_owned()))?;
        if !res.success() {
            return Err(Error::RunCommand("gcc -c".to_owned()))?;
        }
        Ok(())
    }

    pub fn link(&mut self) -> Result<(), Error> {
        if !self.object_out.exists() {
            self.assemble()?;
        }
        let runtime_out = get_runtime_object_out();
        if !runtime_out.exists() {
            self.assemble_runtime()?;
        }

        let mut gcc_cmd = Command::new("gcc");
        gcc_cmd
            .arg(&self.object_out)
            .arg(&runtime_out)
            .arg("-o")
            .arg(&self.exe_out);
        let res = gcc_cmd
            .status()
            .map_err(|_| Error::RunCommand("gcc".to_owned()))?;
        if !res.success() {
            return Err(Error::RunCommand("gcc".to_owned()));
        }
        Ok(())
    }

    pub fn compile(&mut self) -> Result<(), Error> {
        self.parse()?;
        self.uniquify()?;
        self.remove_complex_operands()?;
        self.explicate_control()?;
        self.select_instructions()?;
        self.assign_homes()?;
        self.patch_instructions()?;
        self.generate_prelude_conclusion()?;
        self.write_asm()?;
        self.assemble()?;
        self.link()?;
        Ok(())
    }
}
