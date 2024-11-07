mod core;
mod extract;
mod index;
mod middleware;
mod user;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
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

    let app = core::controller::init();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
