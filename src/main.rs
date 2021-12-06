//! Main module
//!
//! Pattern is often predictable, and anything predictable can be hacked.

use hyper_json::run;

#[tokio::main]
async fn main() {
    run().await.unwrap();
}
