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

    let app = axum_template::core::controller::init().await;
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
