use std::fs;

use crate::{
    config::CONFIG,
    utils::{green_text, hydrate_page, lightblue_text},
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
    fn render_term(&self, ctx: PageContext) -> String {
        let logo = lightblue_text(ROOT_LOGO);

        let legend = [
            format!(
                "{:<60} {:<54} │",
                format_command("curl", &format!("{}/info", &ctx.host)),
                "Get information about TRiGGERPHiSH"
            ),
            format!(
                "{:<60} {:<54} │",
                format_command("curl", &format!("{}/upcoming", &ctx.host)),
                "See upcoming streams and events"
            ),
            format!(
                "{:<60} {:<54} │",
                format_command("curl", &format!("{}/lastseen", &ctx.host)),
                "Check when TRiGGERPHiSH was last online"
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
            ("Website", 0),
            ("Store", 1),
            ("YouTube", 2),
            ("Twitter", 3),
            ("Discord", 4),
            ("Twitch", 5),
            ("TikTok", 6),
            ("Reddit", 7),
        ]
        .iter()
        .map(|(platform, i)| format_social(platform, &ctx.host, about_text[*i]))
        .collect();

        format!(
            r#"{}

	A little program for TRiGGERPHiSH 🎮🌊

	┌─ Legend ──────────────────────────────────────────────────────────────────────────────┐
	│ {}
	└───────────────────────────────────────────────────────────────────────────────────────┘

	┌─ Social Media ───────────────────────────┐   ┌─ About ────────────────────────────────┐
	│ {}
	└──────────────────────────────────────────┘   └────────────────────────────────────────┘

       This site is entirely fan-made • Source: {}"#,
            logo,
            legend.join("\n\t│ "),
            social_media.join("\n\t│ "),
            lightblue_text(&CONFIG.git_repo),
        )
    }

    fn render_html(&self, ctx: PageContext) -> String {
        let page = self.render_term(ctx);
        let file = fs::read_to_string("assets/index.html").unwrap();

        hydrate_page(&page, &file)
    }
}

fn format_command(command: &str, path: &str) -> String {
    format!("{} {}", green_text(command), lightblue_text(path))
}

fn format_social(platform: &str, host: &str, description: &str) -> String {
    if platform.is_empty() {
        format!("{:<40} │   │ {:<38} │", "", description)
    } else {
        format!(
            "{:<9} {:<45} │   │ {:<38} │",
            platform,
            lightblue_text(&format!("{}/{}", format_url(host), platform.to_lowercase())),
            description
        )
    }
}

fn format_url(host: &str) -> String {
    let scheme = if CONFIG.public_host == "triggerphi.sh" { "https" } else { "http" };
    format!("{}://{}", scheme, host)
}
