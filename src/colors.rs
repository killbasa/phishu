pub fn light_blue_text(text: &str) -> String {
    format!("\x1b[38;5;117m{}\x1b[0m", text)
}

pub fn green_text(text: &str) -> String {
    format!("\x1b[38;5;120m{}\x1b[0m", text)
}

pub fn bright_red_text(text: &str) -> String {
    format!("\x1b[38;5;196m{}\x1b[0m", text)
}

pub fn bright_yellow_text(text: &str) -> String {
    format!("\x1b[38;5;226m{}\x1b[0m", text)
}

pub fn bright_purple_text(text: &str) -> String {
    format!("\x1b[38;5;129m{}\x1b[0m", text)
}
