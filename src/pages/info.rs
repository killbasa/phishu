use crate::youtube;

use super::{PageContext, Render};

pub struct Page {}

impl Render for Page {
    fn render_term(&self, _ctx: PageContext) -> String {
        youtube::get_channel_from_api()
    }

    fn render_html(&self, ctx: PageContext) -> String {
        self.render_term(ctx)
    }
}
