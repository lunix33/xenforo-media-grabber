use anyhow::Result;
use scraper::{element_ref::Select, Html};

use crate::{
    content::Content,
    selectors::{SelectorsStore, CONTAINER, ELEMENTS},
};

pub struct Page {
    html: Html,
    selectors_store: SelectorsStore,
}

impl Page {
    pub fn new(content: &[u8], selectors_store: SelectorsStore) -> Self {
        let document = String::from_utf8_lossy(content);
        let html = Html::parse_document(&document);

        Self {
            html,
            selectors_store,
        }
    }

    pub fn iter(&self) -> PageIter {
        let container_selector = self.selectors_store.get(CONTAINER).unwrap();
        let child_selector = self.selectors_store.get(ELEMENTS).unwrap();

        PageIter(
            self.html
                .select(container_selector)
                .next()
                .unwrap()
                .select(child_selector),
            self.selectors_store.clone(),
        )
    }
}

pub struct PageIter<'a>(Select<'a, 'a>, SelectorsStore);

impl<'a> Iterator for PageIter<'a> {
    type Item = Result<Content>;
    // type Item = ElementRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            None => None,
            Some(next) => Some(Content::from_element(next, self.1.clone())),
        }
    }
}
