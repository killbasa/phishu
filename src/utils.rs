use crate::{config::CONFIG, constants};

fn lightblue_text(text: &str) -> String {
    format!("{}{}\x1b[0m", constants::LIGHT_BLUE_ANSI, text)
}

fn green_text(text: &str) -> String {
    format!("{}{}\x1b[0m", constants::GREEN_ANSI, text)
}

fn format_command(command: &str, path: &str) -> String {
    format!("{} {}", green_text(command), lightblue_text(path))
}

fn format_social(platform: &str, host: &str, description: &str) -> String {
    if platform.is_empty() {
        format!("{:<40} │   │ {:<38} │", "", description)
    } else {
        let scheme = if CONFIG.public_host == "triggerphi.sh" { "https" } else { "http" };

        format!(
            "{:<9} {:<45} │   │ {:<38} │",
            platform,
            lightblue_text(&format!("{}://{}/{}", scheme, host, platform.to_lowercase())),
            description
        )
    }
}

pub fn root_banner(host: &str) -> String {
    let logo = lightblue_text(LOGO);

    let legend = [
        format!(
            "{:<60} {:<54} │",
            format_command("curl", &format!("{}/info", host)),
            "Get information about TRiGGERPHiSH"
        ),
        format!(
            "{:<60} {:<54} │",
            format_command("curl", &format!("{}/upcoming", host)),
            "See upcoming streams and events"
        ),
        format!(
            "{:<60} {:<54} │",
            format_command("curl", &format!("{}/lastseen", host)),
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
        ("YouTube", 1),
        ("Twitter", 2),
        ("Discord", 3),
        ("Twitch", 4),
        ("TikTok", 5),
        ("Reddit", 6),
        ("", 7),
    ]
    .iter()
    .map(|(platform, i)| format_social(platform, host, about_text[*i]))
    .collect();

    format!(
        r#"{}

    A little program for TRiGGERPHiSH 🎮🌊

    Source: {}

    ┌─ Legend ──────────────────────────────────────────────────────────────────────────────┐
    │ {}
    └───────────────────────────────────────────────────────────────────────────────────────┘

    ┌─ Social Media ───────────────────────────┐   ┌─ About ────────────────────────────────┐
    │ {}
    └──────────────────────────────────────────┘   └────────────────────────────────────────┘"#,
        logo,
        lightblue_text(&CONFIG.git_repo),
        legend.join("\n    │ "),
        social_media.join("\n    │ ")
    )
}

const LOGO: &str = r#"
    ████████╗██████╗ ██╗ ██████╗  ██████╗ ███████╗██████╗ ██████╗ ██╗  ██╗██╗███████╗██╗  ██╗
    ╚══██╔══╝██╔══██╗██║██╔════╝ ██╔════╝ ██╔════╝██╔══██╗██╔══██╗██║  ██║██║██╔════╝██║  ██║
       ██║   ██████╔╝██║██║  ███╗██║  ███╗█████╗  ██████╔╝██████╔╝███████║██║███████╗███████║
       ██║   ██╔══██╗██║██║   ██║██║   ██║██╔══╝  ██╔══██╗██╔═══╝ ██╔══██║██║╚════██║██╔══██║
       ██║   ██║  ██║██║╚██████╔╝╚██████╔╝███████╗██║  ██║██║     ██║  ██║██║███████║██║  ██║
       ╚═╝   ╚═╝  ╚═╝╚═╝ ╚═════╝  ╚═════╝ ╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝  ╚═╝╚═╝╚══════╝╚═╝  ╚═╝"#;
