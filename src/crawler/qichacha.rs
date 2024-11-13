use std::env;

use serde::Deserialize;

use crate::{core::ApiErr, crawler::HttpClient};

#[derive(Deserialize, Debug)]
struct QichachaResult {
    name: String,
    credit_code: String,
    status: String,
}

// 企查查搜索函数
pub async fn qichacha_search(query: &str) -> Result<(), ApiErr> {
    let api_key = env::var("QICHACHA_API_KEY").expect("QICHACHA_API_KEY must be set");
    let secret_key = env::var("QICHACHA_SECRET_KEY").expect("QICHACHA_SECRET_KEY must be set");
    let endpoint = env::var("QICHACHA_ENDPOINT").expect("QICHACHA_ENDPOINT must be set");

    let client = HttpClient::default().client;

    let url = format!("{}?key={}&keyword={}", endpoint, api_key, query);

    let response = client
        .get(&url)
        .send()
        .await?
        .json::<QichachaResult>()
        .await?;

    println!("Qichacha Result: {:?}", response);
    Ok(())
}
