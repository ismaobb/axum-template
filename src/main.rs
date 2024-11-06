mod core;
mod extract;
mod index;
mod middleware;
mod user;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_init();
    let app = core::controller::init();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

fn tracing_init() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .compact()
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
}
