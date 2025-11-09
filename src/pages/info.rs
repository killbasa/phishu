use anyhow::{Ok, Result};

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
*/

impl Render for Page {
    async fn render_term(&self, _ctx: PageContext) -> Result<String> {
        let channel = youtube::channels::get_channel_api().await?;

        let channel_section = format!(
            "\n{0: <15}{1}\n{2: <15}{3}\n{4: <15}{5}\n{6: <15}{7}\n{8: <15}{9}",
            "name",
            &channel.name.green(),
            "url",
            &CONFIG.vtuber.channel_url.light_blue(),
            "id",
            &CONFIG.vtuber.id.green(),
            "subs",
            &channel.subscriber_count.green(),
            "videos",
            &channel.video_count.green()
        );

        let information_section = format!(
            "\n{0: <15}{1}\n{2: <15}{3}\n{4: <15}{5}\n{6: <15}{7}\n{8: <15}{9}\n{10: <15}{11}",
            "debut",
            "2024/12/13".green(),
            "birthday",
            "June 8th".green(),
            "height",
            "153 cm (5')".green(),
            "weight",
            "48 kg".green(),
            "zodiac",
            "Gemini".green(),
            "emoji",
            "🎮🌊"
        );

        let hashtags_section = format!(
            "\n{0: <15}{1}\n{2: <15}{3}\n{4: <15}{5}\n{6: <15}{7}\n{8: <15}{9}",
            "general tag",
            "https://x.com/hashtag/triggerphish".light_blue(),
            "live tag",
            "https://x.com/hashtag/phishislive".light_blue(),
            "fan tag",
            "https://x.com/hashtag/phish_market".light_blue(),
            "clips tag",
            "https://x.com/hashtag/phishnclips".light_blue(),
            "memes tag",
            "https://x.com/hashtag/phishy_business".light_blue()
        );

        let references_section = format!(
            "\n{0: <15}{1}\n{2: <15}{3}\n{4: <15}{5}",
            "reference",
            "https://x.com/TRiGGERPH1SH/status/1867938176536551850".light_blue(),
            "fan",
            "https://x.com/TRiGGERPH1SH/status/1918307975401488611".light_blue(),
            "mascot",
            "https://x.com/TRiGGERPH1SH/status/1918308464868405584".light_blue()
        );

        Ok(format!(
            "Channel\n{channel_section}\n\nInformation\n{information_section}\n\nHashtags\n{hashtags_section}\n\nReferences\n{references_section}"
        ))
    }

    async fn render_html(&self, _ctx: PageContext) -> Result<String> {
        let channel = youtube::channels::get_channel_api().await?;

        let mut html = HTML_STR
            .replace("{{channel_name}}", &channel.name.green_html())
            .replace("{{channel_url}}", &CONFIG.vtuber.channel_url.light_blue_html())
            .replace("{{channel_id}}", &CONFIG.vtuber.id.green_html())
            .replace("{{channel_subs}}", &channel.subscriber_count.green_html())
            .replace("{{channel_videos}}", &channel.video_count.green_html())
            .replace(
                "{{reference}}",
                &"https://x.com/TRiGGERPH1SH/status/1867938176536551850".light_blue_html(),
            )
            .replace(
                "{{mascot}}",
                &"https://x.com/TRiGGERPH1SH/status/1918307975401488611".light_blue_html(),
            )
            .replace(
                "{{fan_reference}}",
                &"https://x.com/TRiGGERPH1SH/status/1918308464868405584".light_blue_html(),
            );

        html = fix_colored_links(&html);

        compose_page(&html, &format!("Info | {}", CONFIG.server.name))
    }
}
