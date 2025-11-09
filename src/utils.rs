use anyhow::Result;
use once_cell::sync::Lazy;

use crate::config::CONFIG;

static ANCHOR_REGEX: Lazy<regex::Regex> = Lazy::new(|| {
    regex::Regex::new(r#"<span (style='color:#\w+')>(https?://[^\s]+)</span>"#).unwrap()
});

pub fn fix_colored_links(html: &str) -> String {
    ANCHOR_REGEX
        .replace_all(html, |caps: &regex::Captures| {
            let color = caps.get(1).unwrap().as_str();
            let link = caps.get(2).unwrap().as_str();

            if link.contains(&CONFIG.domain) {
                format!(r#"<a href="{link}" {color}>{link}</a>"#)
            } else {
                format!(r#"<a href="{link}" target="_blank" {color}>{link}</a>"#)
            }
        })
        .to_string()
}

const ROOT_HTML_STR: &str = include_str!("assets/root.html");

pub fn compose_page(html: &str, title: &str) -> Result<String> {
    let mut root = ROOT_HTML_STR.replace("{{main}}", html);

    root = root.replace("{{domain}}", &CONFIG.domain);

    let scheme = if CONFIG.domain == "triggerphi.sh" { "https" } else { "http" };
    root = root.replace("{{scheme}}", scheme);

    Ok(root.replace("{{title}}", title))
}
