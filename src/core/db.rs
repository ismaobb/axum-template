use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub async fn init_db(url: &str) -> DatabaseConnection {
    let mut opt = ConnectOptions::new(url);
    opt.max_connections(10)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(5))
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(5))
        .max_lifetime(Duration::from_secs(5));

    Database::connect(opt).await.unwrap()
}
