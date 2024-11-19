use serde::Deserialize;
use std::env;

use crate::core::ApiErr;

use super::HttpClient;

#[derive(Debug, Deserialize)]
struct GoogleResult {
	items: Option<Vec<GoogleItem>>,
}

#[derive(Debug, Deserialize)]
struct GoogleItem {
	title: String,
	link: String,
	snippet: String,
}

// Google 搜索函数
pub async fn google_search(query: &str) -> Result<(), ApiErr> {
	let api_key = env::var("GOOGLE_API_KEY").expect("GOOGLE_API_KEY must be set");
	let cx = env::var("GOOGLE_CX_ID").expect("GOOGLE_CX_ID must be set");
	let search_url = env::var("GOOGLE_API_ENDPOINT").expect("GOOGLE_API_ENDPOINT must be set");

	// 创建带超时机制的客户端
	let client = HttpClient::default().client;
	let url = format!("{}?key={}&cx={}&q={}", search_url, api_key, cx, query);

	let response = client.get(url).send().await?.json::<GoogleResult>().await?;

	if let Some(items) = response.items {
		for item in items {
			println!("Title: {}\nLink: {}\nSnippet: {}\n", item.title, item.link, item.snippet);
		}
	} else {
		println!("No results found from Google.");
	}

	Ok(())
}
