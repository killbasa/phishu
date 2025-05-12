use anyhow::Result;

use crate::{
    colors::Colorize,
    config::CONFIG,
    sqlite, time,
    utils::{compose_page, fix_colored_links},
    youtube::YoutubeVideo,
};

use super::{PageContext, Render};

const HTML_STR: &str = include_str!("upcoming.html");

pub struct Page {}

impl Render for Page {
    async fn render_term(&self, _ctx: PageContext) -> Result<String> {
        let videos = sqlite::get_db_videos().unwrap_or_else(|_| {
            tracing::error!("failed to fetch videos from db");
            Vec::new()
        });

        let mut video_list = Vec::<String>::new();

        for video in videos {
            video_list.push(format_video(&video, true));
        }

        Ok(video_list.join("\n"))
    }

    async fn render_html(&self, _ctx: PageContext) -> Result<String> {
        let title = format!("Upcoming streams | {}", CONFIG.vtuber.name);
        let videos = sqlite::get_db_videos().unwrap_or_else(|_| {
            tracing::error!("failed to fetch videos from db");
            Vec::new()
        });

        if videos.is_empty() {
            let html = HTML_STR.replace("{{video_list}}", "no upcoming streams");
            return compose_page(&html, &title);
        }

        let mut video_list = Vec::<String>::new();

        for video in videos {
            video_list.push(format_video_html(&video));
        }

        let mut html = HTML_STR.replace("{{video_list}}", &video_list.join(""));
        html = fix_colored_links(&html);

        compose_page(&html, &title)
    }
}

fn format_video(video: &YoutubeVideo, is_terminal: bool) -> String {
    let status: String = match video.end_time.is_some() {
        true => "[ended]".bright_purple(),
        false => match video.start_time.is_some() {
            true => "[live]".bright_red(),
            false => "[upcoming]".bright_yellow(),
        },
    };

    let title = &video.title.green();
    let url = &format!("https://www.youtube.com/watch?v={}", video.id).light_blue();

    let mut entry = match is_terminal {
        true => {
            format!("{} {}\n ├─       url: {}\n", status, title, url)
        }
        false => {
            format!("{{{{video:{}}}}}\n{} {}\n ├─       url: {}\n", video.id, status, title, url)
        }
    };

    if let Some(start_time) = &video.start_time {
        let (date, diff) = time::humanize(start_time);

        entry.push_str(&format!(
            " └─   started: {}\n",
            &format!("{} UTC ({})", date, diff).light_blue()
        ));
    } else {
        let (date, diff) = time::humanize(&video.scheduled_time);

        entry.push_str(&format!(
            " └─ scheduled: {}\n",
            &format!("{} UTC ({})", date, diff).light_blue()
        ));
    }

    entry
}

fn format_video_html(video: &YoutubeVideo) -> String {
    let status: String = match video.end_time.is_some() {
        true => "[ended]".bright_purple_html(),
        false => match video.start_time.is_some() {
            true => "[live]".bright_red_html(),
            false => "[upcoming]".bright_yellow_html(),
        },
    };
    let url = &format!("https://www.youtube.com/watch?v={}", video.id).light_blue_html();
    let (date, diff) = time::humanize(&video.scheduled_time);

    format!(
        r#"
				<div class="flex-col">
					<img src="https://img.youtube.com/vi/{}/maxresdefault.jpg" style="max-height:250px;margin:2rem auto 0 auto;" />
					<div class="flex-col">
						<span>{} {}</span>
						<span style="padding-left: 4.5rem;white-space: pre"> ├─       url: {}</span>
						<span style="padding-left: 4.5rem;white-space: pre"> └─   started: {}</span>
					</div>
				</div>
				"#,
        &video.id,
        &status,
        &video.title.green_html(),
        url,
        &format!("{} UTC ({})", date, diff).light_blue_html()
    )
}
