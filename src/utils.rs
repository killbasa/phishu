use once_cell::sync::Lazy;

use crate::constants;

pub fn lightblue_text(text: &str) -> String {
    format!("{}{}\x1b[0m", constants::LIGHT_BLUE_ANSI, text)
}

pub fn green_text(text: &str) -> String {
    format!("{}{}\x1b[0m", constants::GREEN_ANSI, text)
}

pub static ANCHOR_REGEX: Lazy<regex::Regex> = Lazy::new(|| {
    regex::Regex::new(r#"<span (style='color:#\w+')>(https?://[^\s]+)</span>"#).unwrap()
});

const HTML_STR: &str = include_str!("assets/index.html");
pub fn hydrate_page(page: &str) -> String {
    let page_with_color = ansi_to_html::convert(page).unwrap();

    let page_with_links = ANCHOR_REGEX.replace_all(&page_with_color, |caps: &regex::Captures| {
        let color = caps.get(1).unwrap().as_str();
        let link = caps.get(2).unwrap().as_str();

        format!(r#"<a href="{1}" target="_blank" {0}>{1}</a>"#, color, link)
    });

    HTML_STR.replace("{{content}}", &page_with_links.replace("\t", ""))
}
