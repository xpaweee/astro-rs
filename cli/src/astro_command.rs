use async_trait::async_trait;
use integrations::gitlab_client::get_runners;

#[async_trait]
pub trait AstroCommand{
    async fn execute(&self) -> Result<(), ()>;
}