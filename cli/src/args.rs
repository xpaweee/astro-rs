use clap:: {
    Args,
    Parser,
    Subcommand
};


#[derive(Debug, Parser)]
#[clap(name="astro",author, version, about)]
pub struct AstroArgs
{
   #[clap(subcommand)]
   pub operation: OperationType
}

#[derive(Debug, Subcommand)]
pub enum OperationType
{
    /// Manage runners
    Runner(RunnerCommand),
}

#[derive(Debug, Args)]
pub struct RunnerCommand
{
    #[clap(subcommand)]
    pub command: RunnerSubcommand
}


#[derive(Debug, Subcommand)]
pub enum RunnerSubcommand
{
    /// Get runner
    Get(GetRunner),

    /// Delete runner
    Delete(DeleteRunner)
}

#[derive(Debug, Args)]
pub struct GetRunner
{
    /// The name of runner
    pub name: String
}

#[derive(Debug, Args)]
pub struct DeleteRunner
{
    pub name: String
}