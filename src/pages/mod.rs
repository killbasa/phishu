use std::{collections::HashMap, sync::Mutex};

use once_cell::sync::OnceCell;

mod info;
mod lastseen;
mod root;
mod upcoming;

#[derive(Clone)]
pub struct PageContext {
    pub host: String,
    pub is_term: bool,
}

pub trait Render {
    async fn render_term(&self, ctx: PageContext) -> String;
    async fn render_html(&self, ctx: PageContext) -> String;
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum Pages {
    Info,
    LastSeen,
    Root,
    Upcoming,
}

static TERM_CACHE: OnceCell<Mutex<HashMap<Pages, String>>> = OnceCell::new();
static HTML_CACHE: OnceCell<Mutex<HashMap<Pages, String>>> = OnceCell::new();

impl Render for Pages {
    async fn render_term(&self, ctx: PageContext) -> String {
        if let Some(term) = TERM_CACHE.get() {
            if let Some(content) = term.lock().unwrap().get(self) {
                tracing::debug!("cache hit for {:?} term", self);
                return content.clone();
            }
        }

        tracing::debug!("cache miss for {:?} term", self);

        let content = match self {
            Pages::Info => info::Page {}.render_term(ctx).await,
            Pages::LastSeen => lastseen::Page {}.render_term(ctx).await,
            Pages::Root => root::Page {}.render_term(ctx).await,
            Pages::Upcoming => upcoming::Page {}.render_term(ctx).await,
        };

        let cache = TERM_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
        cache.lock().unwrap().insert(*self, content.clone());

        content
    }

    async fn render_html(&self, ctx: PageContext) -> String {
        if let Some(term) = HTML_CACHE.get() {
            if let Some(content) = term.lock().unwrap().get(self) {
                tracing::debug!("cache hit for {:?} html", self);
                return content.clone();
            }
        }

        tracing::debug!("cache miss for {:?} html", self);

        let content = match self {
            Pages::Info => info::Page {}.render_html(ctx).await,
            Pages::LastSeen => lastseen::Page {}.render_html(ctx).await,
            Pages::Root => root::Page {}.render_html(ctx).await,
            Pages::Upcoming => upcoming::Page {}.render_html(ctx).await,
        };

        let cache = HTML_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
        cache.lock().unwrap().insert(*self, content.clone());

        content
    }
}
