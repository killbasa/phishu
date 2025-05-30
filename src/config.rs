use std::env;

use once_cell::sync::Lazy;

pub struct ServerConfig {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub log_level: tracing::Level,
}

pub struct YoutubeConfig {
    pub apikey: String,
}

pub struct VtuberSocialsConfig {
    pub twitter: String,
    pub discord: String,
    pub twitch: String,
    pub tiktok: String,
    pub reddit: String,
    pub website: String,
    pub store: String,
}

pub struct VtuberConfig {
    pub id: String,
    pub name: String,
    pub channel_url: String,
    pub socials: VtuberSocialsConfig,
}

pub struct Config {
    pub server: ServerConfig,
    pub youtube: YoutubeConfig,
    pub vtuber: VtuberConfig,
    pub git_repo: String,
    pub domain: String,
}

impl Config {
    fn get() -> Config {
        let level = match env::var("DEBUG_LOG").as_deref() {
            Ok("1") | Ok("true") => tracing::Level::DEBUG,
            _ => tracing::Level::INFO,
        };

        Config {
            git_repo: "https://github.com/killbasa/phishu".to_string(),
            domain: env::var("PHISHU_DOMAIN").unwrap_or("localhost:3000".to_string()),
            server: ServerConfig {
                name: "PHiSHU".to_string(),
                host: env::var("HOST").unwrap_or("127.0.0.1".to_string()),
                port: env::var("PORT").unwrap_or("3000".to_string()).parse().unwrap_or(3000),
                log_level: level,
            },
            youtube: YoutubeConfig {
                apikey: env::var("YOUTUBE_APIKEY").unwrap(), //
            },
            vtuber: VtuberConfig {
                id: "UC9iiZCKQ9jnIM7zZ_mRX_cg".to_string(),
                name: "TRiGGERPHiSH".to_string(),
                channel_url: "https://www.youtube.com/@TRiGGERPHiSH".to_string(),
                socials: VtuberSocialsConfig {
                    twitter: "https://twitter.com/TRiGGERPH1SH".to_string(),
                    discord: "https://discord.com/invite/4GHZZMm4Sp".to_string(),
                    twitch: "https://www.twitch.tv/triggerph1sh".to_string(),
                    tiktok: "https://www.tiktok.com/@triggerphish_al".to_string(),
                    reddit: "https://www.reddit.com/user/TriggerPh1sh/".to_string(),
                    website: "https://aegis-l.ink/talent/triggerphish".to_string(),
                    store: "https://merch.kawaentertainment.com/en-ca/collections/trigg3rph-h"
                        .to_string(),
                },
            },
        }
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(Config::get);
