use colored::Colorize;

use crate::constants;

pub fn get_channel_from_api() -> String {
    let url = "https://www.youtube.com/@TRiGGERPHiSH";

    format!(
        "\n{0: <9}{1}\n{2: <9}{3}\n{4: <9}{5}\n{6: <9}{7}\n{8: <9}{9}",
        "channel",
        "TRiGGERPHiSH【Aegis-Link】".custom_color(constants::LIGHT_BLUE),
        "url",
        url.custom_color(constants::GREEN),
        "id",
        "UC9iiZCKQ9jnIM7zZ_mRX_cg".custom_color(constants::GREEN),
        "subs",
        "100".custom_color(constants::GREEN),
        "videos",
        "100".custom_color(constants::GREEN)
    )
}
