use std::{collections::HashMap, sync::Mutex};

use once_cell::sync::OnceCell;

mod info;
mod lastseen;
mod root;
mod upcoming;

pub struct PageContext {
    pub host: String,
}

pub trait Render {
    fn render_term(&self, ctx: PageContext) -> String;
    fn render_html(&self, ctx: PageContext) -> String;
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
    fn render_term(&self, ctx: PageContext) -> String {
        if let Some(term) = TERM_CACHE.get() {
            if let Some(content) = term.lock().unwrap().get(self) {
                tracing::debug!("cache hit for {:?} term", self);
                return content.clone();
            }
        }

        tracing::debug!("cache miss for {:?} term", self);

        let content = match self {
            Pages::Info => info::Page {}.render_term(ctx),
            Pages::LastSeen => lastseen::Page {}.render_term(ctx),
            Pages::Root => root::Page {}.render_term(ctx),
            Pages::Upcoming => upcoming::Page {}.render_term(ctx),
        };

        let cache = TERM_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
        cache.lock().unwrap().insert(*self, content.clone());

        content
    }

    fn render_html(&self, ctx: PageContext) -> String {
        if let Some(term) = HTML_CACHE.get() {
            if let Some(content) = term.lock().unwrap().get(self) {
                tracing::debug!("cache hit for {:?} html", self);
                return content.clone();
            }
        }

        tracing::debug!("cache miss for {:?} html", self);

        let content = match self {
            Pages::Info => info::Page {}.render_html(ctx),
            Pages::LastSeen => lastseen::Page {}.render_html(ctx),
            Pages::Root => root::Page {}.render_html(ctx),
            Pages::Upcoming => upcoming::Page {}.render_html(ctx),
        };

        let cache = HTML_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
        cache.lock().unwrap().insert(*self, content.clone());

        content
    }
}
