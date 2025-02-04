use chapter1::{eval::interp_lint, parser::parse_l_int};
use std::{fs::read_to_string, path::PathBuf};

#[derive(clap::Args)]
pub struct Args {
    file_path: PathBuf,
}

pub fn exec(args: Args) {
    let mut contents = read_to_string(args.file_path).unwrap();
    let ast = parse_l_int(&mut contents)
        .map_err(|err| err.to_string())
        .unwrap();
    let evaled = interp_lint(ast);
    println!("{evaled}");
}
