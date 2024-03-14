use async_trait::async_trait;

#[async_trait]
pub trait AstroCommand{
    async fn execute(&self) -> Result<(), ()>;
}