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
        format!("\x1b[38;5;117m{self}\x1b[0m")
    }

    fn light_blue_html(&self) -> String {
        format!("<span style='color:#87d7ff'>{self}</span>")
    }

    fn green(&self) -> String {
        format!("\x1b[38;5;120m{self}\x1b[0m")
    }

    fn green_html(&self) -> String {
        format!("<span style='color:#87ff87'>{self}</span>")
    }

    fn bright_red(&self) -> String {
        format!("\x1b[38;5;196m{self}\x1b[0m")
    }

    fn bright_red_html(&self) -> String {
        format!("<span style='color:#ff0000'>{self}</span>")
    }

    fn bright_yellow(&self) -> String {
        format!("\x1b[38;5;226m{self}\x1b[0m")
    }

    fn bright_yellow_html(&self) -> String {
        format!("<span style='color:#ffff00'>{self}</span>")
    }

    fn bright_purple(&self) -> String {
        format!("\x1b[38;5;129m{self}\x1b[0m")
    }

    fn bright_purple_html(&self) -> String {
        format!("<span style='color:#af00ff'>{self}</span>")
    }
}
