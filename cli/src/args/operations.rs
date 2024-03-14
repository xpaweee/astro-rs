use clap::{Parser, Subcommand};
use crate::args::runners::RunnerCommand;
use crate::args::tokens::TokenCommand;

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

    ///Manage tokens
    Token(TokenCommand)
}