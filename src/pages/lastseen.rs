use anyhow::Result;
use askama::Template;

use crate::{
    colors::Colorize,
    config::CONFIG,
    sqlite, time,
    utils::{compose_page, fix_colored_links},
};

use super::{PageContext, Render};

#[derive(Template)]
#[template(path = "lastseen.html", escape = "none")]
struct LastseenTemplate<'a> {
    last_seen: &'a str,
}

pub struct Page {}

// TODO - add a javascript timer, or some way to update the page client-side

impl Render for Page {
    async fn render_term(&self, _ctx: PageContext) -> Result<String> {
        let video = sqlite::get_db_most_recent_video()?;

        if let Some(video) = video {
            let video_url = &format!("https://youtube.com/watch?v={}", video.id).light_blue();

            match video.end_time {
                Some(end) => {
                    let (_date, diff) = time::humanize(&end);
                    return Ok(format!("phish was last seen {} at {}", &diff.green(), &video_url));
                }
                None => {
                    return Ok(format!("phish is live at {}", &video_url));
                }
            }
        }

        Ok("no recent videos".to_string())
    }

    async fn render_html(&self, _ctx: PageContext) -> Result<String> {
        let video = sqlite::get_db_most_recent_video()?;

        let last_seen = match video {
            None => "no recent videos".to_string(),
            Some(video) => {
                let video_url =
                    &format!("https://youtube.com/watch?v={}", video.id).light_blue_html();

                match video.end_time {
                    Some(end) => {
                        let (_date, diff) = time::humanize(&end);
                        format!("phish was last seen {} at {}", &diff.green_html(), &video_url)
                    }
                    None => format!("phish is live at {}", &video_url),
                }
            }
        };

        let html = LastseenTemplate { last_seen: &last_seen };
        let html_fixed = fix_colored_links(&html.render().unwrap());

        compose_page(&html_fixed, &format!("Last seen | {}", CONFIG.server.name))
    }
}
