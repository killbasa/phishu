use anyhow::Result;
use once_cell::sync::Lazy;

use crate::config::CONFIG;

static ANCHOR_REGEX: Lazy<regex::Regex> = Lazy::new(|| {
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

static VIDEO_THUMBNAIL_REGEX: Lazy<regex::Regex> =
    Lazy::new(|| regex::Regex::new(r"\{\{video:(\w+)\}\}").unwrap());

pub fn hydrate_thumbnail(page: &str) -> String {
    VIDEO_THUMBNAIL_REGEX.replace_all(page, |caps: &regex::Captures| {
		let video_id = caps.get(1).unwrap().as_str();
		format!(
			r#"<img src="https://img.youtube.com/vi/{}/maxresdefault.jpg" style="max-height:250px;margin:2rem auto 0 auto;" />"#,
			video_id
		)
	}).to_string()
}
