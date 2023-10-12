use clap:: {
    Args,
    Parser,
    Subcommand,
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct QArgs {
    #[clap(subcommand)]
    pub entity: Entity,
}

#[derive(Debug, Subcommand)]
pub enum Entity {
    /// Compile a .g program
    Compile(CompileCommand),
}

#[derive(Debug, Args)]
pub struct CompileCommand {
    /// The path of the source file to compile
    pub path: String,
}
