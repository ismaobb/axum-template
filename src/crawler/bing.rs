use serde::Deserialize;
use std::env;

use crate::core::ApiErr;

use super::HttpClient;

#[derive(Debug, Deserialize)]
struct BingResult {
    web_pages: Option<BingWebPages>,
}

#[derive(Debug, Deserialize)]
struct BingWebPages {
    value: Vec<BingWebPageValue>,
}

#[derive(Debug, Deserialize)]
struct BingWebPageValue {
    name: String,
    url: String,
    snippet: String,
}

// Bing 搜索函数
pub async fn bing_search(query: &str) -> Result<(), ApiErr> {
    let api_key = env::var("BING_API_KEY").expect("BING_API_KEY must be set");
    let endpoint = env::var("BING_API_ENDPOINT").expect("BING_API_ENDPOINT must be set");

    let client = HttpClient::default().client;
    let url = format!("{}?q={}", endpoint, query);

    let response = client
        .get(url)
        .header("Ocp-Apim-Subscription-Key".to_string(), api_key)
        .send()
        .await?
        .json::<BingResult>()
        .await?;

    if let Some(web_pages) = response.web_pages {
        for page in web_pages.value {
            println!(
                "Title: {}\nURL: {}\nSnippet: {}\n",
                page.name, page.url, page.snippet
            );
        }
    } else {
        println!("No results found from Bing.");
    }

    Ok(())
}
