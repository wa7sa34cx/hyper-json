//! Library module

mod logging;
mod models;

use dotenv::dotenv;
use hyper::{Body, Client, Method, Request, Uri};
use hyper_tls::HttpsConnector;
// use hyper::body::HttpBody as _;
// use tokio::io::{stdout, AsyncWriteExt as _};
// use models::Price;
use tokio::task;

pub async fn run() -> anyhow::Result<()> {
    // Enables logging
    enable_logging!();

    

    

    

    // Await the response...
    // let mut res = client.request(req).await?;
    // let res = client.request(req).await?;

    // Print HTTP status
    // log::info!("Response: {}", res.status());

    // And now...
    // while let Some(chunk) = res.body_mut().data().await {
    //     stdout().write_all(&chunk?).await?;
    // }

    // while let Some(next) = res.data().await {
    //     let chunk = next?;
    //     io::stdout().write_all(&chunk).await?;
    // }

    let mut handles = Vec::new();

    for i in 1..=10 {
        let handle = task::spawn(async move {
            fetch().await.unwrap();
            log::info!("{}", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await?;
    }

    Ok(())
}

async fn fetch() -> anyhow::Result<()> {
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

    // Await the response...
    // let mut res = client.request(req).await?;
    let res = client.request(req).await?;

    log::info!("Response: {}", res.status());

    Ok(())
}

fn get_env(env: &'static str) -> String {
    dotenv().ok();
    std::env::var(env).unwrap_or_else(|_| panic!("Cannot get the {} env variable", env))
}