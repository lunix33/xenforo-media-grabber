use std::{cell::RefCell, sync::Arc, thread::sleep};

use anyhow::Result;
use chrono::{DateTime, Duration, Local};
use reqwest::{blocking::Client, cookie::Jar, Url};

pub struct HttpClient {
    client: Client,
    origin: Url,
    rate_limit: Option<Duration>,
    next: RefCell<DateTime<Local>>,
}

impl HttpClient {
    pub fn new(origin: &str, auth: Option<&str>, rate_limit: Option<Duration>) -> Result<Self> {
        let mut client = Client::builder();

        let origin = origin.parse::<Url>()?;
        if let Some(auth) = auth {
            let jar = Arc::new(Jar::default());
            jar.add_cookie_str(auth, &origin);

            client = client.cookie_provider(jar);
        }

        Ok(Self {
            client: client.build()?,
            origin,
            rate_limit,
            next: RefCell::new(Local::now()),
        })
    }

    pub fn fetch(&self, path: &str, full: bool) -> Result<(Vec<u8>, String)> {
        // Wait until we can run the next query.
        {
            let next = self.next.borrow();
            if *next > Local::now() {
                let wait = (*next - Local::now()).to_std()?;
                println!("Wait for: {wait:?}");
                sleep(wait);
            }
        }

        // Request the page.
        let uri = self.assemble_url(path, full)?;
        println!("Requesting: {uri}");
        let response = self.client.get(uri).send()?;

        // Set the new next timeout.
        if let Some(rate_limit) = self.rate_limit {
            self.next.replace(Local::now() + rate_limit);
        }

        // Decode and return.
        let content_type = response
            .headers()
            .get("content-type")
            .unwrap()
            .to_str()?
            .to_string();
        let bytes = response.bytes()?;
        Ok((bytes.into_iter().collect::<Vec<u8>>(), content_type))
    }

    fn assemble_url(&self, path: &str, full: bool) -> Result<Url> {
        Ok(match full {
            true => path.to_owned(),
            false => format!("{}{}", &self.origin, path),
        }
        .parse::<Url>()?)
    }
}
