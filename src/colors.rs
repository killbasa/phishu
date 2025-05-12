pub trait Colorize {
    fn light_blue(&self) -> String;
    fn light_blue_html(&self) -> String;

    fn green(&self) -> String;
    fn green_html(&self) -> String;

    fn bright_red(&self) -> String;
    fn bright_red_html(&self) -> String;

    fn bright_yellow(&self) -> String;
    fn bright_yellow_html(&self) -> String;

    fn bright_purple(&self) -> String;
    fn bright_purple_html(&self) -> String;
}

impl Colorize for str {
    fn light_blue(&self) -> String {
        format!("\x1b[38;5;117m{}\x1b[0m", self)
    }

    fn light_blue_html(&self) -> String {
        format!("<span style='color:#87d7ff'>{}</span>", self)
    }

    fn green(&self) -> String {
        format!("\x1b[38;5;120m{}\x1b[0m", self)
    }

    fn green_html(&self) -> String {
        format!("<span style='color:#87ff87'>{}</span>", self)
    }

    fn bright_red(&self) -> String {
        format!("\x1b[38;5;196m{}\x1b[0m", self)
    }

    fn bright_red_html(&self) -> String {
        format!("<span style='color:#ff0000'>{}</span>", self)
    }

    fn bright_yellow(&self) -> String {
        format!("\x1b[38;5;226m{}\x1b[0m", self)
    }

    fn bright_yellow_html(&self) -> String {
        format!("<span style='color:#ffff00'>{}</span>", self)
    }

    fn bright_purple(&self) -> String {
        format!("\x1b[38;5;129m{}\x1b[0m", self)
    }

    fn bright_purple_html(&self) -> String {
        format!("<span style='color:#af00ff'>{}</span>", self)
    }
}
