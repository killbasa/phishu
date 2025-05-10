use anyhow::{Ok, Result, anyhow};
use reqwest::{ClientBuilder, header::ACCEPT};

use crate::config::CONFIG;

use super::{YoutubeChannel, utils::ChannelApiResponse};

pub async fn get_channel_api() -> Result<YoutubeChannel> {
    let url = format!(
        "https://www.googleapis.com/youtube/v3/channels?part=id,snippet,statistics&key={}&id={}",
        CONFIG.youtube.apikey, CONFIG.vtuber.id
    );

    let client = ClientBuilder::new()
        .build()? //
        .get(url)
        .header(ACCEPT, "application/json");

    let response = client.send().await?;
    if response.status().as_u16() != 200 {
        return Err(anyhow!(response.status()));
    }

    let body: ChannelApiResponse = response.json().await?;

    let items = body.items.unwrap_or_default();
    if items.is_empty() {
        return Err(anyhow!("channel not found"));
    }

    let raw_channel = items[0].to_owned();

    let channel = YoutubeChannel {
        id: raw_channel.id,
        name: raw_channel.snippet.title,
        description: raw_channel.snippet.description,
        custom_url: raw_channel.snippet.custom_url,
        view_count: raw_channel.statistics.view_count,
        subscriber_count: raw_channel.statistics.subscriber_count,
        video_count: raw_channel.statistics.video_count,
        profile_picture: raw_channel.snippet.thumbnails.medium.url,
    };

    Ok(channel)
}
