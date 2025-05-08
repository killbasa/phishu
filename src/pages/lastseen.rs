use super::{PageContext, Render};

pub struct Page {}

impl Render for Page {
    fn render_term(&self, _ctx: PageContext) -> String {
        "not implemented".to_string()
    }

    fn render_html(&self, ctx: PageContext) -> String {
        self.render_term(ctx)
    }
}
