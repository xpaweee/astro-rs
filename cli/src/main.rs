mod args;

use async_trait::async_trait;
use clap::Parser;
use crate::args::operations::{AstroArgs, OperationType};
use crate::args::runners::RunnerSubcommand;
use crate::args::tokens::{GetToken, SaveToken, TokenSubcommand};
use sdk::cli::runners;
use sdk::cli::tokens::{get_token, list_tokens, save_token};
use integrations::gitlab_client;
use sdk::cli::astro_command::AstroCommand;
use sdk::cli::runners::get_runners;



#[tokio::main]
async fn main() -> Result<(), ()> {

    let args = AstroArgs::parse();

    get_command(args.operation).await;

    // match args.operation
    // {
    //     OperationType::Runner(runner) => match runner.command {
    //         RunnerSubcommand::Get(user) => {
    //             println!("{}", user.name)
    //         }
    //         _ => {}
    //     },
    //     OperationType::Token(_) => {}
    // }

    Ok(())
}




async fn get_command (operation_type: OperationType) -> Result<(),()>
{
    Ok(match operation_type
    {
        OperationType::Runner(runner) => match runner.command {
            RunnerSubcommand::Get(user) => {
                println!("{}", user.name)
            }
            _ => {}
        },
        OperationType::Token(token) => {
            match token.command
            {
                TokenSubcommand::Get(runner) =>
                    {
                        let cmd = get_token::GetToken::new(runner.name);

                        let result = cmd.execute().await;
                        println!("{:?}", result);


                    }

                TokenSubcommand::Save(key) =>
                {

                    let cmd = save_token::SaveToken::new(key.name);

                    let _ = cmd.execute().await;
                }

                TokenSubcommand::List(key) => {
                    let cmd = list_tokens::ListTokens::new();
                    let _ = cmd.execute().await;
                }
                _ => {}
            }
        }
    })
}