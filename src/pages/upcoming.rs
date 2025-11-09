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
        let videos = sqlite::get_db_upcoming_videos().unwrap_or_else(|_| {
            tracing::error!("failed to fetch videos from db");
            Vec::new()
        });

        if videos.is_empty() {
            return Ok("no upcoming streams".to_string());
        }

        let mut video_list = Vec::<String>::new();

        for video in videos {
            video_list.push(format_video(&video));
        }

        Ok(video_list.join("\n"))
    }

    async fn render_html(&self, _ctx: PageContext) -> Result<String> {
        let title = format!("Upcoming streams | {}", CONFIG.server.name);
        let videos = sqlite::get_db_upcoming_videos().unwrap_or_else(|_| {
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

fn format_video(video: &YoutubeVideo) -> String {
    let status: String = match video.end_time.is_some() {
        true => "[ended]".bright_purple(),
        false => match video.start_time.is_some() {
            true => "[live]".bright_red(),
            false => "[upcoming]".bright_yellow(),
        },
    };

    let title = &video.title.green();
    let url = &format!("https://www.youtube.com/watch?v={}", video.id).light_blue();

    let mut entry = format!("{status} {title}\nurl:       {url}\n");

    if let Some(start_time) = &video.start_time {
        let (date, diff) = time::humanize(start_time);

        entry.push_str(&format!("started:   {}\n", &format!("{date} UTC ({diff})")));
    } else {
        let (date, diff) = time::humanize(&video.scheduled_time);

        entry.push_str(&format!("scheduled: {}\n", &format!("{date} UTC ({diff})")));
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

    let time_str = if let Some(start_time) = &video.start_time {
        let (date, diff) = time::humanize(start_time);
        format!("<td>started:</td><td>{}</td>", &format!("{date} UTC ({diff})"))
    } else {
        let (date, diff) = time::humanize(&video.scheduled_time);
        format!("<td>scheduled:</td><td>{}</td>", &format!("{date} UTC ({diff})"))
    };

    format!(
        r#"
			<div class="video-entry">
				<img src="https://i.ytimg.com/vi_webp/{}/maxresdefault.webp" />
				<div class="flex-col">
					<span>{}{}</span>
					<table>
						<colgroup>
							<col style="width: 1px">
							<col>
						</colgroup>
						<tbody>
							<tr>
								<td>url:</td>
								<td>{}</td>
							</tr>
							<tr>{}</tr>
						</tbody>
					</table>
				</div>
			</div>
				"#,
        &video.id,
        &status,
        &video.title.green_html(),
        url,
        time_str
    )
}
