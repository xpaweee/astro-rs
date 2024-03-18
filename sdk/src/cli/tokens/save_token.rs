use async_trait::async_trait;
use crate::cli::astro_command::AstroCommand;

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
    async fn execute(&self) -> Result<(), ()> {
        Ok(println!("{}", self.key))
    }
}