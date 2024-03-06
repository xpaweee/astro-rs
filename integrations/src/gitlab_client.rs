use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Runner
{
    pub id: u32,
    pub description: Option<String>,
    pub status: String,
    pub online: bool,
    pub ip_address: String,
    pub name: Option<String>
}
const GITLAB_API_URL: &str = "https://gitlab.com/api/v4/runners";

pub async fn get_runners() -> Result<Vec<Runner>, Error>{
    let client = reqwest::Client::new();

    let private_token = "";

    let response = client.get(GITLAB_API_URL)
        .header("PRIVATE-TOKEN", private_token)
        .send()
        .await?
        .json::<Vec<Runner>>()
        .await?;

    Ok(response)
}
