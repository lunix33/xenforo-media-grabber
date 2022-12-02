use std::{collections::HashMap, sync::Arc};

use scraper::Selector;

pub const CONTAINER: &str = "container";
pub const ELEMENTS: &str = "elements";
pub const TITLE: &str = "title";
pub const DATE: &str = "date";
pub const DOWNLOAD: &str = "download";

pub type SelectorsStore = Arc<HashMap<&'static str, Selector>>;

pub struct SelectorsBuilder(HashMap<&'static str, Selector>);

impl SelectorsBuilder {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn compile(mut self, name: &'static str, selector_str: &str) -> Self {
        let selector = Selector::parse(selector_str).unwrap();
        self.0.insert(name, selector);

        self
    }

    pub fn done(mut self) -> SelectorsStore {
        if !self.0.contains_key(CONTAINER) {
            let default_container = Selector::parse("div.LightboxContainer").unwrap();
            self.0.insert(CONTAINER, default_container);
        }

        if !self.0.contains_key(ELEMENTS) {
            let default_element = Selector::parse(":scope > div").unwrap();
            self.0.insert(ELEMENTS, default_element);
        }

        if !self.0.contains_key(TITLE) {
            let default_title = Selector::parse(".titleSection a").unwrap();
            self.0.insert(TITLE, default_title);
        }

        if !self.0.contains_key(DATE) {
            let default_date =
                Selector::parse(".secondaryContent > div > div:first-child").unwrap();
            self.0.insert(DATE, default_date);
        }

        if !self.0.contains_key(DOWNLOAD) {
            let default_download = Selector::parse(".secondaryContent > div > a").unwrap();
            self.0.insert(DOWNLOAD, default_download);
        }

        Arc::new(self.0)
    }
}
