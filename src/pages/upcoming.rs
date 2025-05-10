use chrono::{DateTime, Local};
use chrono_humanize::HumanTime;

use crate::{
    config::CONFIG,
    utils::{
        VIDEO_THUMBNAIL_REGEX, bright_red_text, bright_yellow_text, green_text, hydrate_page,
        light_blue_text,
    },
    youtube::{self, YoutubeVideo},
};

use super::{PageContext, Render};

pub struct Page {}

impl Render for Page {
    async fn render_term(&self, ctx: PageContext) -> String {
        let video_list = get_videos(ctx.is_term).await;
        video_list.join("\n")
    }

    async fn render_html(&self, ctx: PageContext) -> String {
        let video_list = get_videos(ctx.is_term).await;
        let title = format!("Upcoming videos | {}", CONFIG.vtuber.name);

        if video_list.is_empty() {
            return hydrate_page(&ctx.host, "no upcoming videos", &title);
        }

        let page = hydrate_page(&ctx.host, &video_list.join("\n"), &title);

        VIDEO_THUMBNAIL_REGEX
            .replace_all(&page, |caps: &regex::Captures| {
                let video_id = caps.get(1).unwrap().as_str();
                format!(
                    r#"<img src="https://img.youtube.com/vi/{}/maxresdefault.jpg" style="max-height:250px;margin:2rem auto 0 auto;" />"#,
                    video_id
                )
            })
            .to_string()
    }
}

async fn get_videos(is_term: bool) -> Vec<String> {
    let videos = youtube::videos::get_videos_api().await;
    if let Err(e) = videos {
        tracing::error!("Failed to fetch videos: {}", e);
        return vec![];
    }

    let mut uvideos = videos.unwrap();
    uvideos.sort_by(|a, b| {
        let a_time = a.start_time.as_ref().unwrap_or(&a.scheduled_time);
        let b_time = b.start_time.as_ref().unwrap_or(&b.scheduled_time);

        a_time.cmp(b_time)
    });

    let mut video_list = Vec::<String>::new();

    for video in uvideos {
        video_list.push(format_video(&video, is_term));
    }

    video_list
}

pub fn format_video(video: &YoutubeVideo, is_terminal: bool) -> String {
    let status: String = match video.start_time.is_some() {
        true => bright_red_text("[live]"),
        false => bright_yellow_text("[upcoming]"),
    };

    let title = green_text(&video.title);
    let url = light_blue_text(&format!("https://www.youtube.com/watch?v={}", video.id));

    let mut entry = match is_terminal {
        true => {
            format!("{} {}\n ├─       url: {}\n", status, title, url)
        }
        false => {
            format!("{{{{video:{}}}}}\n{} {}\n ├─       url: {}\n", video.id, status, title, url)
        }
    };

    if let Some(start_time) = &video.start_time {
        let (date, diff) = humanize_time(start_time);

        entry.push_str(&format!(
            " └─   started: {}\n",
            light_blue_text(&format!("{} ({})", date, diff))
        ));
    } else {
        let (date, diff) = humanize_time(&video.scheduled_time);

        entry.push_str(&format!(
            " └─ scheduled: {}\n",
            light_blue_text(&format!("{} ({})", date, diff))
        ));
    }

    entry
}

const TIME_FORMAT: &str = "%Y-%m-%d %H:%M";
pub fn humanize_time(time: &str) -> (String, String) {
    let tz = &Local::now().timezone();
    let parsed = DateTime::parse_from_rfc3339(time).unwrap();
    let humanized = HumanTime::from(parsed);

    (
        parsed.with_timezone(tz).format(TIME_FORMAT).to_string(), //
        humanized.to_string(),
    )
}
