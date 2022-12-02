use std::{
    path::{Path, PathBuf},
    rc::Rc,
};

use anyhow::Result;
use chrono::Duration;

mod clap_parser;
mod client;
mod content;
mod page;
mod selectors;
use clap::Parser;
use clap_parser::*;
use client::*;
use content::*;
use page::*;
use rand::Rng;
use selectors::{SelectorsBuilder, SelectorsStore};

const DATE_FMT: &str = "%d %b %Y";

fn main() -> Result<()> {
    let args = CliArgs::parse();

    let selectors_store = SelectorsBuilder::new().done();

    let client = Rc::new(HttpClient::new(
        &args.origin,
        args.session.as_deref(),
        args.rate.map(|ms| Duration::milliseconds(ms)),
    )?);

    let out = PathBuf::from(args.out);
    if args.start.is_some() && args.end.is_some() {
        for page in args.start.unwrap()..=args.end.unwrap() {
            let url = args.path.clone().replace("{}", &page.to_string());
            run_page(client.clone(), &url, &out, selectors_store.clone())?;
        }
    } else {
        run_page(client.clone(), &args.path, &out, selectors_store.clone())?;
    }

    Ok(())
}

fn run_page(
    client: Rc<HttpClient>,
    url: &str,
    out: &Path,
    selectors_store: SelectorsStore,
) -> Result<()> {
    println!("Processing: {url}");

    let (data, _) = client.fetch(url, false)?;
    let page = Page::new(&data, selectors_store.clone());
    for content in page.iter() {
        println!("Found: {content:?}");
        let content = content?;
        let path = to_path(&out, &content)?;

        //println!("Saving request content to: {path}");
        content.download(client.clone(), &path)?;

        println!("")
    }

    println!("Done with: {url}");

    Ok(())
}

fn to_path(base: &Path, content: &Content) -> Result<PathBuf> {
    let mut rand = rand::thread_rng();

    let path = base.join(format!(
        "{}-{}_{}_{}",
        content.date.format("%Y"),
        content.date.format("%m"),
        content.title,
        rand.gen_range(0..100)
    ));

    Ok(path)
}
