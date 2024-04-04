use async_trait::async_trait;
use crate::cli::astro_command::AstroCommand;
use crate::errors::AstroError;

pub struct SaveToken
{
    key: String
}


impl SaveToken
{
    pub fn new(key: String) -> Self
    {
        Self
        {
            key
        }
    }
}

#[async_trait]
impl AstroCommand for SaveToken
{
    async fn execute(&self) -> Result<(), AstroError> {
        Ok(println!("{}", self.key))
    }
}