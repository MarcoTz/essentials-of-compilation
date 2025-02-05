use clap::{Parser, Subcommand};

mod lif;
mod lint;
mod lvar;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    LInt(lint::Args),
    LVar(lvar::Args),
    LIf(lif::Args),
}

pub fn exec() {
    let cli = Cli::parse();
    match cli.command {
        Command::LInt(args) => lint::exec(args),
        Command::LVar(args) => lvar::exec(args),
        Command::LIf(args) => lif::exec(args),
    }
}
