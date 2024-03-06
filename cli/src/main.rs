mod args;
mod runners;
mod astro_command;
use args::AstroArgs;
use clap::Parser;
use runners::get_runners::GetRunners;
use crate::astro_command::AstroCommand;


// async  fn main() -> Result<(), ()>
// {
//     let args = AstroArgs::parse();
//
//     Ok(())
// }

#[tokio::main]
async fn main() {

    let cmd = GetRunners { };
    cmd.execute().await.unwrap();
    let args = AstroArgs::parse();


}