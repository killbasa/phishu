use std::{collections::HashMap, sync::Mutex};

use anyhow::Result;
use once_cell::sync::OnceCell;

mod info;
mod lastseen;
mod root;
mod upcoming;

#[derive(Clone)]
pub struct PageContext {
    pub is_term: bool,
}

pub trait Render {
    async fn render_term(&self, ctx: PageContext) -> Result<String>;
    async fn render_html(&self, ctx: PageContext) -> Result<String>;
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum Pages {
    Info,
    LastSeen,
    Root,
    Upcoming,
}

// TODO - cleanup locks

static TERM_CACHE: OnceCell<Mutex<HashMap<Pages, String>>> = OnceCell::new();
static HTML_CACHE: OnceCell<Mutex<HashMap<Pages, String>>> = OnceCell::new();

pub async fn refresh_page(page: Pages) -> Result<()> {
    if let Some(term) = TERM_CACHE.get() {
        term.lock().unwrap().remove(&page);

        tracing::debug!("cache cleared for {:?} term", page);
        page.render_term(PageContext { is_term: true }).await?;
    }

    if let Some(html) = HTML_CACHE.get() {
        html.lock().unwrap().remove(&page);

        tracing::debug!("cache cleared for {:?} html", page);
        page.render_html(PageContext { is_term: false }).await?;
    }

    Ok(())
}

impl Render for Pages {
    async fn render_term(&self, ctx: PageContext) -> Result<String> {
        if let Some(term) = TERM_CACHE.get() {
            if let Some(content) = term.lock().unwrap().get(self) {
                tracing::debug!("cache hit for {:?} term", self);
                return Ok(content.clone());
            }
        }

        tracing::debug!("cache miss for {:?} term", self);

        let content = match self {
            Pages::Info => info::Page {}.render_term(ctx).await?,
            Pages::LastSeen => lastseen::Page {}.render_term(ctx).await?,
            Pages::Root => root::Page {}.render_term(ctx).await?,
            Pages::Upcoming => upcoming::Page {}.render_term(ctx).await?,
        };

        let cache = TERM_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
        cache.lock().unwrap().insert(*self, content.clone());

        Ok(content)
    }

    async fn render_html(&self, ctx: PageContext) -> Result<String> {
        if let Some(term) = HTML_CACHE.get() {
            if let Some(content) = term.lock().unwrap().get(self) {
                tracing::debug!("cache hit for {:?} html", self);
                return Ok(content.clone());
            }
        }

        tracing::debug!("cache miss for {:?} html", self);

        let content = match self {
            Pages::Info => info::Page {}.render_html(ctx).await?,
            Pages::LastSeen => lastseen::Page {}.render_html(ctx).await?,
            Pages::Root => root::Page {}.render_html(ctx).await?,
            Pages::Upcoming => upcoming::Page {}.render_html(ctx).await?,
        };

        let cache = HTML_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
        cache.lock().unwrap().insert(*self, content.clone());

        Ok(content)
    }
}
