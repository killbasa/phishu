use anyhow::Result;
use chrono_humanize::HumanTime;
use once_cell::sync::Lazy;

use crate::{
    config::CONFIG,
    constants::{BRIGHT_RED_ANSI, BRIGHT_YELLOW_ANSI, GREEN_ANSI, LIGHT_BLUE_ANSI},
};

pub fn light_blue_text(text: &str) -> String {
    format!("{}{}\x1b[0m", LIGHT_BLUE_ANSI, text)
}

pub fn green_text(text: &str) -> String {
    format!("{}{}\x1b[0m", GREEN_ANSI, text)
}

pub fn bright_red_text(text: &str) -> String {
    format!("{}{}\x1b[0m", BRIGHT_RED_ANSI, text)
}

pub fn bright_yellow_text(text: &str) -> String {
    format!("{}{}\x1b[0m", BRIGHT_YELLOW_ANSI, text)
}

pub static VIDEO_THUMBNAIL_REGEX: Lazy<regex::Regex> =
    Lazy::new(|| regex::Regex::new(r"\{\{video:(\w+)\}\}").unwrap());

pub static ANCHOR_REGEX: Lazy<regex::Regex> = Lazy::new(|| {
    regex::Regex::new(r#"<span (style='color:#\w+')>(https?://[^\s]+)</span>"#).unwrap()
});

const HTML_STR: &str = include_str!("assets/index.html");
pub fn hydrate_page(page: &str, title: &str) -> Result<String> {
    let page_with_color = ansi_to_html::convert(page).unwrap();

    let page_with_links = ANCHOR_REGEX.replace_all(&page_with_color, |caps: &regex::Captures| {
        let color = caps.get(1).unwrap().as_str();
        let link = caps.get(2).unwrap().as_str();

        if link.contains(&CONFIG.public_host) {
            format!(r#"<a href="{1}" {0}>{1}</a>"#, color, link)
        } else {
            format!(r#"<a href="{1}" target="_blank" {0}>{1}</a>"#, color, link)
        }
    });

    let shifted = &page_with_links.replace("\t", "");
    let with_content = HTML_STR.replace("{{content}}", shifted);

    Ok(with_content.replace("{{title}}", title))
}

const TIME_FORMAT: &str = "%Y-%m-%d %H:%M";
pub fn humanize_time(time: &str) -> (String, String) {
    let tz = &chrono::Utc::now().timezone();
    let parsed = chrono::DateTime::parse_from_rfc3339(time).unwrap();
    let humanized = HumanTime::from(parsed);

    (
        parsed.with_timezone(tz).format(TIME_FORMAT).to_string(), //
        humanized.to_string(),
    )
}
