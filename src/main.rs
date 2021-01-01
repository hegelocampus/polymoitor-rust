use std::result::Result;
use structopt::StructOpt;
use ureq::get;
use url::{ParseError, Url};

#[derive(Debug, StructOpt)]
#[structopt(name = "Polymonitor")]
/// Simple http response monitor built to be used with Polybar
struct Opt {
    /// Reduces the results into a more compact package
    #[structopt(short, long)]
    compact: bool,
    /// Displays the results as symbols
    #[structopt(short, long)]
    symbolic: bool,
    /// Pass in URLS to monitor
    urls: Vec<String>,
}

struct ValMap<'a> {
    up: &'a str,
    down: &'a str,
}

fn make_value_map<'a>(symbolic: bool) -> ValMap<'a> {
    if symbolic {
        ValMap {
            up: "\u{f062}",
            down: "\u{f98d}",
        }
    } else {
        ValMap {
            up: "Up",
            down: "Down",
        }
    }
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
    // Send get request, check for error for down url and warn if so
    let res = get(&url).call();
    if res.error() {
        println!("{} bad status: {}", url, res.status_text());
    }

    res.ok()
}

fn main() -> Result<(), &'static str> {
    let Opt {
        compact,
        symbolic,
        urls,
    } = Opt::from_args();

    if urls.is_empty() {
        return Err("Please pass in valid urls you would like to monitor");
    } else {
        let statuses = urls.iter().map(parse_url).map(|url| {
            let owned = url.to_owned();
            let stat = get_status(&owned);
            (owned, stat)
        });

        let val_map = make_value_map(symbolic);

        let parsed_output = if compact {
            let (up_count, down_urls) = statuses.fold(
                (0, Vec::new()),
                |(up_count, mut down_urls), (url, is_up)| {
                    if is_up {
                        (up_count + 1, down_urls)
                    } else {
                        down_urls.push(url);
                        (up_count, down_urls)
                    }
                },
            );
            let down_parsed = if down_urls.len() == 0 {
                "0".to_string()
            } else {
                down_urls.join(", ")
            };
            format!(
                "{}: {}, {}: {}",
                val_map.up, up_count, val_map.down, down_parsed
            )
        } else {
            statuses
                .map(|(url, status)| {
                    let parsed_status = if status { val_map.up } else { val_map.down };
                    format!("{}: {}", url, parsed_status).to_owned()
                })
                .collect::<Vec<String>>()
                .join(", ")
        };

        println!("{}", parsed_output);

        Ok(())
    }
}
