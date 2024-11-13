#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expire_in_hours: i64,
    pub whitelist: Option<Vec<String>>,
    pub basic_auth_username: String,
    pub basic_auth_password: String,
}

impl Config {
    pub fn init() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            jwt_secret: std::env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set"),
            jwt_expire_in_hours: std::env::var("JWT_EXPIRE_IN_HOURS")
                .expect("JWT_EXPIRE_IN_HOURS must be set")
                .parse()
                .expect("JWT_EXPIRE_IN_HOURS must be a valid integer"),
            whitelist: std::env::var("WHITELIST_PATHS")
                .ok()
                .map(|s| s.split(",").map(|s| s.trim().to_owned()).collect()),
            basic_auth_username: std::env::var("BASIC_AUTH_USERNAME")
                .expect("BASIC_AUTH_USERNAME must be set"),
            basic_auth_password: std::env::var("BASIC_AUTH_PASSWORD")
                .expect("BASIC_AUTH_PASSWORD must be set"),
        }
    }
}
