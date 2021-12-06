//! Library module

mod logging;

use dotenv::dotenv;
use hyper::{Client, Uri};

pub async fn run() -> anyhow::Result<()> {
    // Create client
    let client = Client::new();

    // Get URI from .env and parse it
    let uri: Uri = get_env("ENDPOINT").parse()?;

    // Await the response...
    let resp = client.get(uri).await?;

    // Print HTTP status
    log::info!("Response: {}", resp.status());

    Ok(())
}

fn get_env(env: &'static str) -> String {
    dotenv().ok();
    std::env::var(env).unwrap_or_else(|_| panic!("Cannot get the {} env variable", env))
}