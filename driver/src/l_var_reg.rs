use super::{
    assemble::{assemble, link_obj, write_asm},
    consts::{ASSEMBLY_DIR, C_DIR, EXE_DIR, LIB_C, L_VAR_REG_DIR, OBJ_DIR, OUT_DIR},
    l_var::LVarDriver,
    Driver,
};
use chapter3::{
    assign_homes::assign_homes,
    color_graph::{color_graph, coloring_to_string},
    interference_graph::build_graph,
    patch_instructions::patch_instructions,
    prelude_conclusion::generate_prelude_conclusion,
};
use std::{fs::create_dir_all, path::PathBuf};

pub struct LVarRegDriver {
    print_intermediary: bool,
    asm_dir: PathBuf,
    obj_dir: PathBuf,
    exe_dir: PathBuf,
    lib_c: PathBuf,

    l_var_driver: LVarDriver,
}

impl LVarRegDriver {
    pub fn new(print_intermediary: bool) -> LVarRegDriver {
        let drv = LVarRegDriver {
            print_intermediary,
            asm_dir: PathBuf::from(OUT_DIR)
                .join(L_VAR_REG_DIR)
                .join(ASSEMBLY_DIR),
            obj_dir: PathBuf::from(OUT_DIR).join(L_VAR_REG_DIR).join(OBJ_DIR),
            exe_dir: PathBuf::from(OUT_DIR).join(L_VAR_REG_DIR).join(EXE_DIR),
            lib_c: PathBuf::from(C_DIR).join(LIB_C),
            l_var_driver: LVarDriver::new(print_intermediary),
        };
        create_dir_all(&drv.asm_dir).unwrap();
        create_dir_all(&drv.obj_dir).unwrap();
        create_dir_all(&drv.exe_dir).unwrap();
        drv
    }
}

impl Driver for LVarRegDriver {
    type Target = PathBuf;
    type Parsed = chapter2::l_var::Program;

    fn is_debug(&self) -> bool {
        self.print_intermediary
    }

    fn parse(&self, input: &str) -> Result<Self::Parsed, Box<dyn std::error::Error>> {
        self.l_var_driver.parse(input)
    }

    fn compile(
        &self,
        input: Self::Parsed,
        prog_name: String,
    ) -> Result<Self::Target, Box<dyn std::error::Error>> {
        let prog = self.l_var_driver.compile_lvar(input)?;
        self.debug(&prog.to_string());

        let inter_graph = build_graph(&prog);
        self.debug("----- Interference Graph ----");
        self.debug(&inter_graph.to_string());

        let coloring = color_graph(inter_graph);
        self.debug("----- Graph Coloring ----");
        self.debug(&coloring_to_string(&coloring));

        let prog = assign_homes(prog, coloring);
        self.debug("----- Assigned Homes ----");
        self.debug(&prog.to_string());

        let patched = patch_instructions(prog);
        self.debug("----- Patched Instructions ----");
        self.debug(&patched.to_string());

        let prel_conc = generate_prelude_conclusion(patched);
        self.debug("----- Generated Prelude and Conclusion ----");
        self.debug(&prel_conc.to_string());

        let asm_file = write_asm(prel_conc.to_string(), prog_name.clone(), &self.asm_dir)?;
        self.debug(&format!("Successfully wrote {:?}", asm_file));

        let obj_file = assemble(&asm_file, &self.obj_dir)?;
        self.debug(&format!("Successfully wrote {:?}", obj_file));

        let linked_file = link_obj(&obj_file, &self.exe_dir, &self.lib_c)?;
        self.debug(&format!("Successfully wrote {:?}", linked_file));

        Ok(linked_file)
    }

    fn evaluate(&self, _input: Self::Target) -> Result<String, Box<dyn std::error::Error>> {
        todo!()
    }
}

impl Default for LVarRegDriver {
    fn default() -> LVarRegDriver {
        LVarRegDriver::new(false)
    }
}
