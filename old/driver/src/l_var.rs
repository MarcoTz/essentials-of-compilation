use super::{
    assemble::{assemble, link_obj, write_asm},
    consts::{ASSEMBLY_DIR, C_DIR, EXE_DIR, LIB_C, L_VAR_DIR, OBJ_DIR, OUT_DIR},
    Driver,
};
use chapter2::{
    assign_homes::AssignHomes,
    c_var::typecheck::typecheck,
    explicate_control::ExplicateControl,
    l_var::uniquify::Uniquify,
    parser::parse_program,
    remove_complex_operands::RemoveComplexOperands,
    select_instructions::SelectInstructions,
    x86_int::{
        patch_instructions::PatchInstructions, prelude_conclusion::generate_prelude_conclusion,
    },
};
use core::str;
use std::{fs::create_dir_all, path::PathBuf, process::Command};

pub struct LVarDriver {
    print_intermediary: bool,
    asm_dir: PathBuf,
    obj_dir: PathBuf,
    exe_dir: PathBuf,
    lib_c: PathBuf,
}

impl LVarDriver {
    pub fn new(print_intermediary: bool) -> LVarDriver {
        let drv = LVarDriver {
            print_intermediary,
            asm_dir: PathBuf::from(OUT_DIR).join(L_VAR_DIR).join(ASSEMBLY_DIR),
            obj_dir: PathBuf::from(OUT_DIR).join(L_VAR_DIR).join(OBJ_DIR),
            exe_dir: PathBuf::from(OUT_DIR).join(L_VAR_DIR).join(EXE_DIR),
            lib_c: PathBuf::from(C_DIR).join(LIB_C),
        };

        create_dir_all(&drv.asm_dir).unwrap();
        create_dir_all(&drv.obj_dir).unwrap();
        create_dir_all(&drv.exe_dir).unwrap();
        drv
    }
}

impl LVarDriver {
    pub fn compile_lvar(
        &self,
        input: chapter2::l_var::Program,
    ) -> Result<chapter2::x86_var::Program, Box<dyn std::error::Error>> {
        let prog_unique = input.uniquify(&mut Default::default());
        self.debug("------ Uniquified -----");
        self.debug(&prog_unique.to_string());

        let prog_reduced = prog_unique.remove_complex_operands(&mut Default::default());
        self.debug("------ Reduced -----");
        self.debug(&prog_reduced.to_string());

        let mut prog_explicated = prog_reduced.explicate_control();
        self.debug("------ Explicated -----");
        self.debug(&prog_explicated.to_string());

        typecheck(&mut prog_explicated);
        self.debug("------ Typechecked -----");
        self.debug(&prog_explicated.to_string());

        let selected = prog_explicated.select_instructions();
        self.debug("------ Selected Instructions -----");
        self.debug(&selected.to_string());
        Ok(selected)
    }
}

impl Driver for LVarDriver {
    type Target = PathBuf;
    type Parsed = chapter2::l_var::Program;

    fn is_debug(&self) -> bool {
        self.print_intermediary
    }

    fn parse(&self, input: &str) -> Result<Self::Parsed, Box<dyn std::error::Error>> {
        let (_, parsed) = parse_program(input)?;
        self.debug("----- Parsed ----");
        self.debug(&parsed.to_string());
        Ok(parsed)
    }

    fn compile(
        &self,
        input: Self::Parsed,
        prog_name: String,
    ) -> Result<Self::Target, Box<dyn std::error::Error>> {
        let prog_selected = self.compile_lvar(input)?;

        let prog_homes = prog_selected.assign_homes(&mut Default::default());
        self.debug("------ Assigned Homes -----");
        self.debug(&prog_homes.to_string());

        let prog_patched = prog_homes.patch();
        self.debug("------ Patched Instructions -----");
        self.debug(&prog_patched.to_string());

        let prog_prel_conc = generate_prelude_conclusion(prog_patched);
        self.debug("------ Generated Prelude and Conclusion -----");
        self.debug(&prog_prel_conc.to_string());

        let asm_file = write_asm(prog_prel_conc.to_string(), prog_name.clone(), &self.asm_dir)?;
        self.debug(&format!("Successfully wrote {:?}", asm_file));

        let obj_file = assemble(&asm_file, &self.obj_dir)?;
        self.debug(&format!("Successfully wrote {:?}", obj_file));

        let linked_file = link_obj(&obj_file, &self.exe_dir, &self.lib_c)?;
        self.debug(&format!("Successfully wrote {:?}", linked_file));

        Ok(linked_file)
    }

    fn evaluate(&self, bin_file: Self::Target) -> Result<String, Box<dyn std::error::Error>> {
        Ok(str::from_utf8(&Command::new(bin_file).output()?.stdout)?.to_owned())
    }
}

impl Default for LVarDriver {
    fn default() -> LVarDriver {
        LVarDriver::new(false)
    }
}
