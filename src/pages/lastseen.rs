use anyhow::Result;

use crate::{
    config::CONFIG,
    sqlite::get_db_most_recent_video,
    utils::{self, hydrate_page},
};

use super::{PageContext, Render};

pub struct Page {}

// TODO - add a javascript timer, or some way to update the page client-side

impl Render for Page {
    async fn render_term(&self, _ctx: PageContext) -> Result<String> {
        let video = get_db_most_recent_video()?;

        if let Some(video) = video {
            match video.end_time {
                Some(end) => {
                    let (_date, diff) = utils::humanize_time(&end);
                    return Ok(format!("phish was last seen {}", diff));
                }
                None => {
                    return Ok(format!(
                        "phish is live at https://youtube.com/watch?v={} !",
                        video.id
                    ));
                }
            }
        }

        Ok("no recent videos".to_string())
    }

    async fn render_html(&self, ctx: PageContext) -> Result<String> {
        let page = self.render_term(ctx.clone()).await?;

        hydrate_page(&page, &format!("Last seen | {}", CONFIG.vtuber.name))
    }
}
