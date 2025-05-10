pub mod channels;
mod utils;
pub mod videos;
mod xml;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct YoutubeChannel {
    pub id: String,
    pub name: String,
    pub description: String,
    pub custom_url: Option<String>,
    pub view_count: String,
    pub subscriber_count: String,
    pub video_count: String,
    pub profile_picture: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct YoutubeVideo {
    pub id: String,
    pub title: String,
    pub scheduled_time: String,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}
