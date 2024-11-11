use std::{env, time::Duration};

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub async fn init_db() -> DatabaseConnection {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    tracing::info!(db_url);
    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(10)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(5))
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(5))
        .max_lifetime(Duration::from_secs(5));

    Database::connect(opt).await.unwrap()
}
