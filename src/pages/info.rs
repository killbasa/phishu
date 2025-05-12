use anyhow::Result;

use crate::{
    colors::Colorize,
    config::CONFIG,
    utils::{compose_page, fix_colored_links},
    youtube,
};

use super::{PageContext, Render};

const HTML_STR: &str = include_str!("info.html");

pub struct Page {}

/*
TODO	add info from wiki page
        goals
        birthday
        fan name
        ref sheets
*/

impl Render for Page {
    async fn render_term(&self, _ctx: PageContext) -> Result<String> {
        let channel = youtube::channels::get_channel_api().await?;

        Ok(format!(
            "\n{0: <9}{1}\n{2: <9}{3}\n{4: <9}{5}\n{6: <9}{7}\n{8: <9}{9}",
            "channel",
            &channel.name.green(),
            "url",
            &CONFIG.vtuber.channel_url.light_blue(),
            "id",
            &CONFIG.vtuber.id.green(),
            "subs",
            &channel.subscriber_count.green(),
            "videos",
            &channel.video_count.green()
        ))
    }

    async fn render_html(&self, _ctx: PageContext) -> Result<String> {
        let channel = youtube::channels::get_channel_api().await?;

        let mut html = HTML_STR
            .replace("{{channel_name}}", &channel.name.green_html())
            .replace("{{channel_url}}", &CONFIG.vtuber.channel_url.light_blue_html())
            .replace("{{channel_id}}", &CONFIG.vtuber.id.green_html())
            .replace("{{channel_subs}}", &channel.subscriber_count.green_html())
            .replace("{{channel_videos}}", &channel.video_count.green_html());

        html = fix_colored_links(&html);

        compose_page(&html, &format!("Info | {}", CONFIG.vtuber.name))
    }
}
