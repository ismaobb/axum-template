use std::time::Duration;

use reqwest::{Client, Proxy};
use tokio::join;

mod bing;
mod google;
mod qichacha;
mod tianyancha;

struct HttpClient {
	client: Client,
}

impl Default for HttpClient {
	fn default() -> Self {
		Self {
			client: Client::builder()
				.timeout(Duration::from_secs(3))
				.proxy(Proxy::http("http://127.0.0.1:7890").unwrap())
				.build()
				.expect("Failed to build http client"),
		}
	}
}

pub async fn parallel_search(query: &str) {
	let (bing, google, qichacha, tianyancha) = join!(
		bing::bing_search(query),
		google::google_search(query),
		qichacha::qichacha_search(query),
		tianyancha::tianyancha_search(query)
	);

	match bing {
		Ok(bing) => println!("bing: {:?}", bing),
		Err(e) => tracing::error!("bing error: {:?}", e),
	}
	match google {
		Ok(google) => println!("google: {:?}", google),
		Err(e) => tracing::error!("google error: {:?}", e),
	}
	match qichacha {
		Ok(qichacha) => println!("qichacha: {:?}", qichacha),
		Err(e) => tracing::error!("qichacha error: {:?}", e),
	}
	match tianyancha {
		Ok(tianyancha) => println!("tianyancha: {:?}", tianyancha),
		Err(e) => tracing::error!("tianyancha error: {:?}", e),
	}
}
