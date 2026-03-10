use anyhow::Result;
use askama::Template;
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

#[derive(Template)]
#[template(path = "root.html", escape = "none")]
struct RootTemplate<'a> {
    title: &'a str,
    main: &'a str,
}

pub fn compose_page(html: &str, title: &str) -> Result<String> {
    let root = RootTemplate {
        title, //
        main: html,
    };

    Ok(root.render()?)
}
