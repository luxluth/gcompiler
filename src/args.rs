use clap:: {
    Args,
    Parser,
    Subcommand,
};

use clap_stdin::MaybeStdin;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct QArgs {
    #[clap(subcommand)]
    pub entity: Entity,
}

#[derive(Debug, Subcommand)]
pub enum Entity {
    /// Compile a .g program from a source file
    Compile(CompileCommand),

    /// Compile a .g program from input string
    Raw(RawCommand),
}

#[derive(Debug, Args)]
pub struct CompileCommand {
    /// The path of the source file to compile
    pub path: String,
}

#[derive(Debug, Args)]
pub struct RawCommand {
    /// The input string to compile
    pub input: MaybeStdin<String>,
}
