use once_cell::sync::Lazy;

pub static LIGHT_BLUE_ANSI: &str = "\x1b[38;5;117m";
pub static GREEN_ANSI: &str = "\x1b[38;5;120m";
pub static BRIGHT_RED_ANSI: &str = "\x1b[38;5;196m";
pub static BRIGHT_YELLOW_ANSI: &str = "\x1b[38;5;226m";

pub static HTML_CSP: Lazy<String> = Lazy::new(|| {
    [
        "default-src 'none'", //
        "style-src-elem 'nonce-html-style'",
        "style-src-attr 'unsafe-inline'",
        "img-src 'self' https://img.youtube.com",
    ]
    .join("; ")
});
