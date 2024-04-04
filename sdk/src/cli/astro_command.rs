use async_trait::async_trait;
use crate::errors::AstroError;

#[async_trait]
pub trait AstroCommand{
    async fn execute(&self) -> Result<(), AstroError>;
}