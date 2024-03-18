use async_trait::async_trait;
use crate::cli::astro_command::AstroCommand;
use crate::persistence::token_manager;
#[derive(Debug)]
pub struct GetToken
{
    name: String
}

impl GetToken
{
    pub fn new(name: String) -> Self
    {
        Self
        {
            name
        }
    }
}

#[async_trait]
impl AstroCommand for GetToken
{
    async fn execute(&self) -> Result<(), ()> {
        println!("xd");
        let token_manager = token_manager::TokenManager { };
        let result = token_manager.get_token(self.name.as_str())?;
        println!("{}", result);

        Ok(())
    }
}
