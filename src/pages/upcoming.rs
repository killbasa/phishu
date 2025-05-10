use anyhow::Result;

use crate::{
    config::CONFIG,
    sqlite,
    utils::{
        self, VIDEO_THUMBNAIL_REGEX, bright_red_text, bright_yellow_text, green_text, hydrate_page,
        light_blue_text,
    },
    youtube::YoutubeVideo,
};

use super::{PageContext, Render};

pub struct Page {}

impl Render for Page {
    async fn render_term(&self, ctx: PageContext) -> Result<String> {
        let video_list = get_videos(ctx.is_term).await;

        Ok(video_list.join("\n"))
    }

    async fn render_html(&self, ctx: PageContext) -> Result<String> {
        let video_list = get_videos(ctx.is_term).await;
        let title = format!("Upcoming streams | {}", CONFIG.vtuber.name);

        if video_list.is_empty() {
            return hydrate_page("no upcoming streams", &title);
        }

        let page = hydrate_page(&video_list.join("\n"), &title)?;

        Ok(VIDEO_THUMBNAIL_REGEX
            .replace_all(&page, |caps: &regex::Captures| {
                let video_id = caps.get(1).unwrap().as_str();
                format!(
                    r#"<img src="https://img.youtube.com/vi/{}/maxresdefault.jpg" style="max-height:250px;margin:2rem auto 0 auto;" />"#,
                    video_id
                )
            })
            .to_string())
    }
}

async fn get_videos(is_term: bool) -> Vec<String> {
    let mut videos = sqlite::get_db_videos().unwrap_or_else(|_| {
        tracing::error!("failed to fetch videos from db");
        Vec::new()
    });

    videos.sort_by(|a, b| {
        let a_time = a.start_time.as_ref().unwrap_or(&a.scheduled_time);
        let b_time = b.start_time.as_ref().unwrap_or(&b.scheduled_time);

        a_time.cmp(b_time)
    });

    let mut video_list = Vec::<String>::new();

    for video in videos {
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
        let (date, diff) = utils::humanize_time(start_time);

        entry.push_str(&format!(
            " └─   started: {}\n",
            light_blue_text(&format!("{} ({})", date, diff))
        ));
    } else {
        let (date, diff) = utils::humanize_time(&video.scheduled_time);

        entry.push_str(&format!(
            " └─ scheduled: {}\n",
            light_blue_text(&format!("{} ({})", date, diff))
        ));
    }

    entry
}
