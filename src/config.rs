use std::env;

use once_cell::sync::Lazy;

pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

pub struct Config {
    pub server: ServerConfig,
    pub git_repo: String,
    pub public_host: String,
}

impl Config {
    fn get() -> Config {
        Config {
            server: ServerConfig {
                host: env::var("HOST").unwrap_or("127.0.0.1".to_string()), //
                port: env::var("PORT").unwrap_or("3000".to_string()).parse().unwrap_or(3000),
            },
            git_repo: "https://github.com/killbasa/phishu".to_string(),
            public_host: env::var("PHISHU_DOMAIN").unwrap_or("localhost:3000".to_string()),
        }
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(Config::get);
