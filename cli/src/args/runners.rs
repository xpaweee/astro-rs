
use clap:: {
    Args,
    Subcommand
};

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
    /// The name of runner
    pub name: String
}