use anyhow::Result;

use crate::{
    colors::{green_text, light_blue_text},
    config::CONFIG,
    utils::hydrate_page,
};

use super::{PageContext, Render};

const ROOT_LOGO: &str = r#"
	████████╗██████╗ ██╗ ██████╗  ██████╗ ███████╗██████╗ ██████╗ ██╗  ██╗██╗███████╗██╗  ██╗
	╚══██╔══╝██╔══██╗██║██╔════╝ ██╔════╝ ██╔════╝██╔══██╗██╔══██╗██║  ██║██║██╔════╝██║  ██║
	   ██║   ██████╔╝██║██║  ███╗██║  ███╗█████╗  ██████╔╝██████╔╝███████║██║███████╗███████║
	   ██║   ██╔══██╗██║██║   ██║██║   ██║██╔══╝  ██╔══██╗██╔═══╝ ██╔══██║██║╚════██║██╔══██║
	   ██║   ██║  ██║██║╚██████╔╝╚██████╔╝███████╗██║  ██║██║     ██║  ██║██║███████║██║  ██║
	   ╚═╝   ╚═╝  ╚═╝╚═╝ ╚═════╝  ╚═════╝ ╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝  ╚═╝╚═╝╚══════╝╚═╝  ╚═╝"#;

pub struct Page {}

impl Render for Page {
    async fn render_term(&self, _ctx: PageContext) -> Result<String> {
        let logo = light_blue_text(ROOT_LOGO);

        let legend = [
            format!(
                "{:<47} {:<52} │",
                light_blue_text(&format_url(&CONFIG.public_host).to_string()),
                "Landing page"
            ),
            format!(
                "{:<47} {:<52} │",
                light_blue_text(&format!("{}/{}", format_url(&CONFIG.public_host), "info")),
                format!("Check information about {}", CONFIG.vtuber.name)
            ),
            format!(
                "{:<47} {:<52} │",
                light_blue_text(&format!("{}/{}", format_url(&CONFIG.public_host), "upcoming")),
                "See upcoming streams"
            ),
            format!(
                "{:<47} {:<52} │",
                light_blue_text(&format!("{}/{}", format_url(&CONFIG.public_host), "lastseen")),
                format!("Check when {} was last online", CONFIG.vtuber.name)
            ),
        ];

        let commands = [
            format!("{:<115} │", format_command("curl", &CONFIG.public_host),),
            format!("{:<115} │", format_command("curl", &format!("{}/info", CONFIG.public_host)),),
            format!(
                "{:<115} │",
                format_command("curl", &format!("{}/upcoming", CONFIG.public_host)),
            ),
            format!(
                "{:<115} │",
                format_command("curl", &format!("{}/lastseen", CONFIG.public_host)),
            ),
        ];

        let about_text = [
            "A lazy NEET with a penchant for FPS",
            "games, she directs most of her energy",
            "toward living life with as little",
            "effort as possible. Preferring typing",
            "over talking, streaming may pose a",
            "challenge to her… but if there's one",
            "thing certain about Phish, it's that",
            "she never backs down from a challenge.",
        ];

        let social_media: Vec<String> = [
            ("YouTube", 0),
            ("Twitter", 1),
            ("Discord", 2),
            ("Twitch", 3),
            ("TikTok", 4),
            ("Reddit", 5),
            ("Website", 6),
            ("Store", 7),
        ]
        .iter()
        .map(|(platform, i)| format_social(platform, about_text[*i]))
        .collect();

        Ok(format!(
            r#"{}

	A little program for TRiGGERPHiSH 🎮🌊

	┌─ Directory ───────────────────────────────────────────────────────────────────────────┐
	│ {}
	└───────────────────────────────────────────────────────────────────────────────────────┘

	┌─ Commands ────────────────────────────────────────────────────────────────────────────┐
	│ You can also view all of the above pages in a terminal using these commands:          │
	│ {}
	└───────────────────────────────────────────────────────────────────────────────────────┘

	┌─ Social Media ───────────────────────────┐   ┌─ About ────────────────────────────────┐
	│ {}
	└──────────────────────────────────────────┘   └────────────────────────────────────────┘

	This site is entirely fan-made • Source: {}"#,
            logo,
            legend.join("\n\t│ "),
            commands.join("\n\t│ "),
            social_media.join("\n\t│ "),
            light_blue_text(&CONFIG.git_repo),
        ))
    }

    async fn render_html(&self, ctx: PageContext) -> Result<String> {
        let page = self.render_term(ctx.clone()).await?;

        hydrate_page(&page, &CONFIG.vtuber.name)
    }
}

fn format_command(command: &str, path: &str) -> String {
    format!("{} {}", green_text(command), light_blue_text(path))
}

fn format_social(platform: &str, description: &str) -> String {
    if platform.is_empty() {
        format!("{:<40} │   │ {:<38} │", "", description)
    } else {
        format!(
            "{:<9} {:<45} │   │ {:<38} │",
            platform,
            light_blue_text(&format!(
                "{}/{}",
                format_url(&CONFIG.public_host),
                platform.to_lowercase()
            )),
            description
        )
    }
}

fn format_url(host: &str) -> String {
    let scheme = if CONFIG.public_host == "triggerphi.sh" { "https" } else { "http" };
    format!("{}://{}", scheme, host)
}
