use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let level = std::env::var("RUST_LOG").unwrap_or(LevelFilter::INFO.to_string());
    std::env::set_var("RUST_LOG", level);

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .compact()
        .pretty()
        .init();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    tracing::info!(
        "{}",
        format!(
            "🚀 Server is running on http://{}",
            listener.local_addr().unwrap()
        )
    );

    let _ = axum_template::crawler::parallel_search("%E4%B8%AD%E5%9B%BD%E5%8D%8E%E8%9E%8D%E8%B5%84%E4%BA%A7%E7%AE%A1%E7%90%86%E8%82%A1%E4%BB%BD%E6%9C%89%E9%99%90%E5%85%AC%E5%8F%B8%20%E8%B3%84%20OR%20%E9%9D%9E%E6%B3%95%20OR%20%E6%BF%AB%20OR%20%E5%81%87%E5%B8%B3%20OR%20%E8%AA%BF%E6%9F%A5%20OR%20%E5%AE%98%E5%8F%B8%20OR%20%E9%81%95%E5%8F%8D%20OR%20%E8%AA%B9%E8%AC%97%20OR%20%E9%86%9C%E8%81%9E%20OR%20%E7%88%AD%E8%AD%B0%20OR%20%E7%8D%84%20OR%20%E8%AA%A4%E5%B0%8E%20OR%20%E6%93%8D%E7%B8%B1%20OR%20%E6%93%8D%E6%8E%A7%20OR%20%E9%BB%91%E7%A4%BE%E6%9C%83%20OR%20%E9%BB%91%E9%8C%A2%20OR%20%E8%B3%AD%20OR%20%E6%8C%87%E6%8E%A7%20OR%20%E6%8E%A7%E5%91%8A%20OR%20%E7%9B%9C%20OR%20%E7%AB%8A%20OR%20%E7%BD%AA%20OR%20%E6%AC%BA%20OR%20%E9%A8%99%20OR%20%E6%90%B6%20OR%20%E5%A7%A6%20OR%20%E5%AB%96%20OR%20%E6%B7%AB%20OR%20%E5%BB%89%E6%94%BF%E5%85%AC%E7%BD%B2%20OR%20%E5%BB%89%E7%BD%B2%20OR%20%E8%AD%89%E7%9B%A3%E6%9C%83%20OR%20%E9%99%A4%E7%89%8C%20OR%20%E8%AD%A6%E5%91%8A%20OR%20%E8%AD%A6%E6%88%92%20OR%20%E9%81%BF%E7%A8%85%20OR%20%E9%80%83%E7%A8%85%20OR%20%E5%85%A7%E5%B9%95%20OR%20%E8%AD%89%E6%9C%9F%E5%B1%80%20OR%20%E5%8F%8D%E8%B2%AA%20OR%20%E6%87%B2%E6%88%92%20OR%20%E8%A3%81%E7%BD%B0%20OR%20%E8%99%95%E5%88%86%20OR%20%E8%88%9E%E5%BC%8A%20OR%20%E8%A8%B4%E8%A8%9F%20OR%20%E6%8C%87%E8%B2%AC%20OR%20%E5%88%A4%E6%B1%BA%20OR%20%E6%8A%95%E8%A8%B4%20OR%20%E5%AF%A9%E5%88%A4%20OR%20%E9%96%8B%E5%BA%AD%20OR%20%E9%9B%99%E8%A6%8F%20OR%20%E6%AA%A2%E5%AF%9F%E5%AE%98%20OR%20%E7%9B%A3%E5%AF%9F%20OR%20%E9%BB%91%E7%AE%B1%20OR%20%E9%81%95%E6%B3%95%20OR%20%E9%81%95%E7%B4%80%20OR%20%E8%99%95%E7%BD%B0%20OR%20%E8%99%9B%E5%81%87%20OR%20%E5%88%91%E4%BA%8B%20OR%20%E7%B3%BE%E7%B4%9B%20OR%20%E6%A1%88%E4%BB%B6%20OR%20%E8%85%90%E6%95%97%20OR%20%E6%92%A4%E8%81%B7%20OR%20%E5%81%9C%E8%81%B7%20OR%20%E8%89%B2%E6%83%85%20OR%20%E5%A5%B4%E9%9A%B8%20OR%20%E5%BE%9E%E5%99%A8%E4%BB%B6%20OR%20%E5%BC%B7%E8%BF%AB%E5%8B%9E%E5%8B%95%20OR%20%E5%BC%B7%E5%").await;

    let app = axum_template::core::controller::init().await;
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
