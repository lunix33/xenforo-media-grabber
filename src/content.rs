use std::{path::Path, rc::Rc};

use anyhow::{anyhow, Result};
use chrono::NaiveDate;
use regex::Regex;
use scraper::ElementRef;

use crate::{
    client::HttpClient,
    selectors::{self, SelectorsStore},
    DATE_FMT,
};

#[derive(Debug)]
pub struct Content {
    pub title: String,
    pub date: NaiveDate,
    pub download_uri: String,
}

impl<'a> Content {
    pub fn from_element(ele: ElementRef<'a>, selector_store: SelectorsStore) -> Result<Self> {
        Ok(Self {
            title: Self::parse_title(&ele, &selector_store)?,
            date: Self::parse_date(&ele, &selector_store)?,
            download_uri: Self::parse_download(&ele, &selector_store)?,
        })
    }

    fn parse_title(ele: &ElementRef<'a>, selector_store: &SelectorsStore) -> Result<String> {
        let title_selector = selector_store.get(selectors::TITLE).unwrap();
        let replace_rgx = Regex::new(r"\W+")?;
        let title = ele
            .select(title_selector)
            .next()
            .ok_or(anyhow!("Unable to find title element"))?
            .text()
            .next()
            .ok_or(anyhow!("Unable to find title text"))?
            .to_owned();
        Ok(replace_rgx.replace_all(&title, "-").to_string())
    }

    fn parse_date(ele: &ElementRef<'a>, selector_store: &SelectorsStore) -> Result<NaiveDate> {
        let date_selector = selector_store.get(selectors::DATE).unwrap();

        let date_str = ele
            .select(date_selector)
            .next()
            .ok_or(anyhow!("Date element not found."))?
            .text()
            .next()
            .ok_or(anyhow!("Date text not found."))?
            .trim();

        Ok(NaiveDate::parse_from_str(date_str, DATE_FMT)?)
    }

    fn parse_download(ele: &ElementRef<'a>, selector_store: &SelectorsStore) -> Result<String> {
        let download_selector = selector_store.get(selectors::DOWNLOAD).unwrap();

        Ok(ele
            .select(download_selector)
            .next()
            .ok_or(anyhow!("Unable to find download element"))?
            .value()
            .attr("href")
            .ok_or(anyhow!("Unable to find download href"))?
            .to_owned())
    }

    pub fn download(&self, client: Rc<HttpClient>, to: &Path) -> Result<()> {
        let (data, content_type) = client.fetch(&self.download_uri, false)?;
        let ext = match content_type {
            ct if ct.contains("jpeg") => "jpg",
            ct if ct.contains("png") => "png",
            _ => "",
        };
        let mut path_ext = to.to_path_buf();
        path_ext.set_extension(ext);
        println!("Saving request content to: {}", path_ext.to_string_lossy());

        std::fs::create_dir_all(path_ext.parent().unwrap())?;
        std::fs::write(path_ext, data)?;
        Ok(())
    }
}
