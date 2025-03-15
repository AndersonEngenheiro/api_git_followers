use reqwest::Error;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Error>{
    let user = "andersonengenheiro";

    let request_url = format!("https://api.github.com/users/{}/followers", user);

    println!("{}", request_url);

    let client = reqwest::Client::new();
    let response = client
            .get(&request_url)
            .header("User-Agent", "Rust-reqwest")
            .send()
            .await?;
    
    let json: Value = response.json().await?;

    println!("{}", serde_json::to_string_pretty(&json).unwrap());

    Ok(())

}