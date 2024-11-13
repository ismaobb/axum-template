use std::time::Duration;

use reqwest::Client;

pub mod bing;
pub mod google;
pub mod qichacha;
pub mod tianyancha;

struct HttpClient {
    client: Client,
}

impl Default for HttpClient {
    fn default() -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(5))
                .build()
                .expect("Failed to build http client"),
        }
    }
}
