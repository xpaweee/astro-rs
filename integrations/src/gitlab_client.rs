use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize)]
struct Runner
{
    id: u32,
    description: String,
    ip_address: String,
    name: String
}


const GITLAB_API_URL: &str = "https://gitlab.com/api/v4/projects";

async fn get_runners() -> Result<(), Error>{
    let client = reqwest::Client::new();

    let private_token = "";

    let req_url = format!("{}?private_token={}", GITLAB_API_URL, private_token);

    let response = client.get(&req_url)
        .send()
        .await?
        .json::<Vec<Runner>>()
        .await?;


    for runner in response {
        println!("Name: {}, ID: {}, URL: {}", runner.name, runner.id, runner.ip_address);
        if let Some(description) = runner.description {
            println!("Description: {}", description);
        }
        println!();
    }

    Ok(())
}
