use clap::{Args, Subcommand};


#[derive(Debug, Args)]
pub struct TokenCommand
{
    #[clap(subcommand)]
    pub command: TokenSubcommand
}

#[derive(Debug, Subcommand)]
pub enum TokenSubcommand
{
    /// Get token
    Get(GetToken),

    /// Delete token
    Delete(DeleteToken)
}


#[derive(Debug, Args)]
pub struct GetToken
{
    /// The name of token
    pub name: String
}

#[derive(Debug, Args)]
pub struct DeleteToken
{
    /// The name of token
    pub name: String
}