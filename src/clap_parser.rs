use clap::Parser;

#[derive(Debug, Parser)]
pub struct CliArgs {
    #[arg(
        long,
        help = "The base URL of the website. Example: https://my-website.com/"
    )]
    pub origin: String,
    #[arg(
        long,
        help = "The path of the gallery within the website, uses `{}` as a replacement pattern for the page number. Example: media/?page={}"
    )]
    pub path: String,
    #[arg(long, help = "The first page to grab.")]
    pub start: Option<i32>,
    #[arg(long, help = "The last page to grab.")]
    pub end: Option<i32>,
    #[arg(long, help = "The output directory for the downloaded files.")]
    pub out: String,
    #[arg(
        long,
        help = "The session cookie used by your browser. You'll need to use the dev tools of your browser and grab your session cookie. Example: xf_session=f862807a987b2aa197e1fc208f75cc30"
    )]
    pub session: Option<String>,
    #[arg(
        long,
        help = "The number of ms between requests. Useful if the website doesn't allow quick requests."
    )]
    pub rate: Option<i64>,
}
