use std::result::Result;
use structopt::StructOpt;
use ureq::get;
use url::{ParseError, Url};

#[derive(Debug, StructOpt)]
#[structopt(name = "Polymonitor")]
/// Simple http response monitor built to be used with Polybar
struct Opt {
    /// Displays the results as symbols
    #[structopt(short, long)]
    symbolic: bool,
    /// Reduces the results into a more compact package
    #[structopt(short, long)]
    compact: bool,
    /// Pass in URLS to monitor
    urls: Vec<String>,
}

fn parse_url(url: &String) -> String {
    match Url::parse(&url) {
        Err(ParseError::InvalidIpv6Address) | Err(ParseError::RelativeUrlWithoutBase) => {
            let mut prefix: String = "https://".to_owned();
            prefix.push_str(&url);
            prefix
        }
        _ => url.to_string(),
    }
}

fn get_status(url: &str) -> bool {
    // Send get request, check for reqwest error for bad url and warn if so
    let res = get(&url).call();
    if res.error() {
        println!("{:?}", res.status_text());
    }

    res.ok()
}

fn map_statuses(urls: Vec<String>) -> Vec<(String, bool)> {
    urls.into_iter()
        .map(|url| {
            let owned = url.to_owned();
            let stat = get_status(&owned);
            (owned, stat)
        })
        .collect()
}

fn main() -> Result<(), &'static str> {
    let args = Opt::from_args();

    if args.urls.is_empty() {
        return Err("Please pass in valid urls you would like to monitor");
    } else {
        let parsed_urls = args.urls.iter().map(parse_url).collect();
        println!("{:?}", parsed_urls);
        let status = map_statuses(parsed_urls);
        println!("{:?}", status);

        Ok(())
    }
}
