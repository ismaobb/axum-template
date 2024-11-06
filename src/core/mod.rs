use config::Config;
use serde::{Deserialize, Serialize};

mod api_err;
mod api_ok;
pub use api_err::ApiErr;
pub use api_ok::ApiOk;

pub mod controller;

mod config;
pub mod jwt;

#[derive(Debug, PartialEq, Eq, Serialize, Clone, Default, Deserialize)]
pub struct Role(pub i32);

#[derive(Debug, Clone)]
pub struct RoleState(pub Role);

#[derive(Debug, Clone)]
pub struct AppState {
    pub conn: u32,
    pub config: Config,
}