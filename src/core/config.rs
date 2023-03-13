use serde::Serialize;
use std::env;

#[derive(Serialize)]
pub struct AppConfig {
    pub port: u16,
}

impl AppConfig {
    pub fn new() -> Self {
        AppConfig {
            port: env::var("PORT")
                .unwrap_or("8000".to_owned())
                .parse()
                .unwrap(),
        }
    }
}
