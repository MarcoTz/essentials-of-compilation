use clap::Parser;
use compile::Compiler;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    in_file: PathBuf,
    #[arg(short, long)]
    out: Option<PathBuf>,
    #[arg(long)]
    object_out: Option<PathBuf>,
    #[arg(long)]
    asm_out: Option<PathBuf>,
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut compiler = Compiler::new(
        args.verbose,
        args.in_file,
        args.asm_out,
        args.object_out,
        args.out,
    )?;
    compiler.assemble()?;
    Ok(())
}
