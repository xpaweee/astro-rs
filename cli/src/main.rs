mod args;

use clap::Parser;
use crate::args::operations::{AstroArgs, OperationType};
use crate::args::runners::RunnerSubcommand;


#[tokio::main]
async fn main() -> Result<(), ()> {

    let args = AstroArgs::parse();

    match args.operation
    {
        OperationType::Runner(runner) => match runner.command {
            RunnerSubcommand::Get(user) => {
                println!("{}", user.name)
            }
            _ => {}
        },
        OperationType::Token(_) => {}
    }

    Ok(())

}