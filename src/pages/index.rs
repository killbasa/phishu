use anyhow::Result;
use askama::Template;

use crate::{colors::Colorize, config::CONFIG, utils::compose_page};

use super::{PageContext, Render};

const ROOT_LOGO: &str = r#"
	████████╗██████╗ ██╗ ██████╗  ██████╗ ███████╗██████╗ ██████╗ ██╗  ██╗██╗███████╗██╗  ██╗
	╚══██╔══╝██╔══██╗██║██╔════╝ ██╔════╝ ██╔════╝██╔══██╗██╔══██╗██║  ██║██║██╔════╝██║  ██║
	   ██║   ██████╔╝██║██║  ███╗██║  ███╗█████╗  ██████╔╝██████╔╝███████║██║███████╗███████║
	   ██║   ██╔══██╗██║██║   ██║██║   ██║██╔══╝  ██╔══██╗██╔═══╝ ██╔══██║██║╚════██║██╔══██║
	   ██║   ██║  ██║██║╚██████╔╝╚██████╔╝███████╗██║  ██║██║     ██║  ██║██║███████║██║  ██║
	   ╚═╝   ╚═╝  ╚═╝╚═╝ ╚═════╝  ╚═════╝ ╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝  ╚═╝╚═╝╚══════╝╚═╝  ╚═╝"#;

#[derive(Template)]
#[template(path = "index.html", escape = "none")]
struct IndexTemplate<'a> {
    scheme: &'a str,
    domain: &'a str,
}

pub struct Page {}

// TODO - Update term page to use the new format

impl Render for Page {
    async fn render_term(&self, _ctx: PageContext) -> Result<String> {
        let logo = ROOT_LOGO.light_blue();

        let legend = [
            format!(
                "{:<47} {:<52} │",
                &format_url(&CONFIG.domain).to_string().light_blue(),
                "Landing page"
            ),
            format!(
                "{:<47} {:<52} │",
                &format!("{}/{}", format_url(&CONFIG.domain), "info").light_blue(),
                format!("Check information about {}", CONFIG.vtuber.name)
            ),
            format!(
                "{:<47} {:<52} │",
                &format!("{}/{}", format_url(&CONFIG.domain), "upcoming").light_blue(),
                "See upcoming streams"
            ),
            format!(
                "{:<47} {:<52} │",
                &format!("{}/{}", format_url(&CONFIG.domain), "lastseen").light_blue(),
                format!("Check when {} was last online", CONFIG.vtuber.name)
            ),
        ];

        let commands = [
            format!("{:<115} │", format_command("curl", &CONFIG.domain),),
            format!("{:<115} │", format_command("curl", &format!("{}/info", CONFIG.domain)),),
            format!("{:<115} │", format_command("curl", &format!("{}/upcoming", CONFIG.domain)),),
            format!("{:<115} │", format_command("curl", &format!("{}/lastseen", CONFIG.domain)),),
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
            ("Website", 5),
            ("Store", 6),
            ("", 7),
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
            &CONFIG.git_repo.light_blue(),
        ))
    }

    async fn render_html(&self, _ctx: PageContext) -> Result<String> {
        let html = IndexTemplate {
            scheme: if CONFIG.domain == "triggerphi.sh" { "https" } else { "http" },
            domain: &CONFIG.domain,
        };

        compose_page(&html.render().unwrap(), &CONFIG.server.name)
    }
}

fn format_command(command: &str, path: &str) -> String {
    format!("{} {}", command.green(), path.light_blue())
}

fn format_social(platform: &str, description: &str) -> String {
    if platform.is_empty() {
        format!("{:<40} │   │ {:<38} │", "", description)
    } else {
        format!(
            "{:<9} {:<45} │   │ {:<38} │",
            platform,
            &format!("{}/{}", format_url(&CONFIG.domain), platform.to_lowercase()).light_blue(),
            description
        )
    }
}

fn format_url(host: &str) -> String {
    let scheme = if CONFIG.domain == "triggerphi.sh" { "https" } else { "http" };
    format!("{scheme}://{host}")
}
