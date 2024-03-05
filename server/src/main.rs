use integrations::gitlab_client::get_runners;

#[tokio::main]
async fn main() {
    match  get_runners().await{
        Ok(_) => {println!("Ok")}
        Err(e) => {println!("Error: {}", e)}
    } {  }
}
