use std::env;

use serde::Deserialize;

use crate::{core::ApiErr, crawler::HttpClient};

#[derive(Deserialize, Debug)]
pub struct TianyanchaResult {
    name: String,
    reg_capital: String,
}

// 天眼查搜索函数
pub async fn tianyancha_search(query: &str) -> Result<TianyanchaResult, ApiErr> {
    let api_key = env::var("TIANYANCHA_API_KEY").expect("TIANYANCHA_API_KEY must be set");
    let endpoint = env::var("TIANYANCHA_ENDPOINT").expect("TIANYANCHA_ENDPOINT must be set");
    let client = HttpClient::default().client;
    let url = format!("{}?word={}", endpoint, query);

    let response = client
        .get(&url)
        .header("Authorization", api_key)
        .send()
        .await?
        .json::<TianyanchaResult>()
        .await?;

    println!("Tianyancha Result: {:?}", response);
    Ok(response)
}
