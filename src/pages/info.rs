use anyhow::Result;

use crate::{
    colors::{green_text, light_blue_text},
    config::CONFIG,
    utils::hydrate_page,
    youtube,
};

use super::{PageContext, Render};

pub struct Page {}

impl Render for Page {
    async fn render_term(&self, _ctx: PageContext) -> Result<String> {
        let channel = youtube::channels::get_channel_api().await?;

        Ok(format!(
            "\n{0: <9}{1}\n{2: <9}{3}\n{4: <9}{5}\n{6: <9}{7}\n{8: <9}{9}",
            "channel",
            light_blue_text(&channel.name),
            "url",
            green_text(&CONFIG.vtuber.channel_url),
            "id",
            green_text(&CONFIG.vtuber.id),
            "subs",
            green_text(&channel.subscriber_count),
            "videos",
            green_text(&channel.video_count)
        ))
    }

    async fn render_html(&self, ctx: PageContext) -> Result<String> {
        let page = self.render_term(ctx.clone()).await?;

        hydrate_page(&page, &format!("Info | {}", CONFIG.vtuber.name))
    }
}
