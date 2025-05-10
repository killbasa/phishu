use crate::{config::CONFIG, utils::hydrate_page};

use super::{PageContext, Render};

pub struct Page {}

impl Render for Page {
    async fn render_term(&self, _ctx: PageContext) -> String {
        "not implemented".to_string()
    }

    async fn render_html(&self, ctx: PageContext) -> String {
        let page = self.render_term(ctx.clone()).await;
        hydrate_page(&ctx.host, &page, &format!("Last seen | {}", CONFIG.vtuber.name))
    }
}
