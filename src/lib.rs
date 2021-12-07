//! Library module

mod logging;
mod models;

use dotenv::dotenv;
use hyper::body::Buf;
use hyper::{Body, Client, Method, Request, Uri};
use hyper_tls::HttpsConnector;
use models::Price;
use std::cmp::Ordering;
use tokio::task;

pub async fn run() -> anyhow::Result<()> {
    // Enables logging
    enable_logging!();

    let mut handles = Vec::new();

    for i in 1..=10 {
        let handle = task::spawn(async move {
            let price = match fetch_json().await {
                Ok(p) => p,
                Err(e) => {
                    log::warn!("Task {}: {}", i, e);
                    return;
                }
            };

            let Price { price, max_price } = price;

            // Compare the price with the maximum purchase price
            match price.cmp(&max_price) {
                Ordering::Less => {
                    log::warn!(
                        "Task {}: price={} is lower than max_price={}",
                        i,
                        price,
                        max_price
                    );
                }
                Ordering::Greater => {
                    log::info!("Task {}: price={} and max_price={}", i, price, max_price);
                }
                Ordering::Equal => {
                    log::warn!(
                        "Task {}: price={} is equal max_price={}",
                        i,
                        price,
                        max_price
                    );
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await?;
    }

    Ok(())
}

/// Fetches data
async fn fetch_json() -> anyhow::Result<Price> {
    // Get URI from .env and parse it
    let uri: Uri = get_env("ENDPOINT").parse()?;

    // Create request
    let req = Request::builder()
        .method(Method::GET)
        .uri(uri)
        .header("User-Agent", "hyper-json/1.0")
        .body(Body::empty())?;

    // Create client
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);

    // Await the response
    let res = client.request(req).await?;

    // Aggregate the chunks of the body
    let body = hyper::body::aggregate(res).await?;

    // try to parse as json with serde_json
    let price = serde_json::from_reader(body.reader())?;

    Ok(price)
}

/// Gets env variables
fn get_env(env: &'static str) -> String {
    dotenv().ok();
    std::env::var(env).unwrap_or_else(|_| panic!("Cannot get the {} env variable", env))
}
