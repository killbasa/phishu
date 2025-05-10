use serde::Deserialize;

// ----- Videos -----

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VideoSnippet {
    pub title: String,
    pub channel_title: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VideoLiveStreamingDetails {
    pub actual_start_time: Option<String>,
    pub actual_end_time: Option<String>,
    pub scheduled_start_time: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RawYoutubeVideo {
    pub id: String,
    pub snippet: VideoSnippet,
    pub live_streaming_details: Option<VideoLiveStreamingDetails>,
}

#[derive(Deserialize, Debug)]
pub struct VideoApiResponse {
    pub items: Vec<RawYoutubeVideo>,
}

// ----- Channels -----

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChannelThumbnail {
    pub url: String,
    // pub width: u32,
    // pub height: u32,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChannelThumbnails {
    // pub default: ChannelThumbnail,
    pub medium: ChannelThumbnail,
    // pub high: ChannelThumbnail,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChannelSnippet {
    pub title: String,
    pub description: String,
    pub custom_url: Option<String>,
    pub thumbnails: ChannelThumbnails,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChannelStatistics {
    pub view_count: String,
    pub subscriber_count: String,
    pub video_count: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawYoutubeChannel {
    pub id: String,
    pub snippet: ChannelSnippet,
    pub statistics: ChannelStatistics,
}

#[derive(Deserialize, Debug)]
pub struct ChannelApiResponse {
    pub items: Option<Vec<RawYoutubeChannel>>,
}
