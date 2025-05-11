use once_cell::sync::Lazy;

pub static HTML_CSP: Lazy<String> = Lazy::new(|| {
    [
        "default-src 'none'", //
        "style-src-elem 'nonce-html-style'",
        "style-src-attr 'unsafe-inline'",
        "img-src 'self' https://img.youtube.com",
    ]
    .join("; ")
});
