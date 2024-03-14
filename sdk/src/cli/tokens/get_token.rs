use crate::cli::astro_command::AstroCommand;
use crate::persistence::token_manager;
struct GetToken
{
    name: String
}

impl GetToken
{
    pub fn new(name: String)
    {
        Self
        {
            name
        };
    }
}

// impl AstroCommand for GetToken
// {
//     async fn execute(&self) -> Result<(), ()> {
//
//         let token_manager = token_manager::TokenManager { };
//         let result = token_manager.get_token(self.name.as_str())?;
//         println!("{}", result);
//
//         Ok(())
//     }
// }
