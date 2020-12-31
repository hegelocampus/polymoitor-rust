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
    // Send get request, check for reqwest error for bad url and warn if so
    let res = get(&url).call();
    if res.error() {
        println!("{:?}", res.status_text());
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
            let count = statuses.fold((0, 0), |acc, cur| {
                if cur.1 {
                    (acc.0 + 1, acc.1)
                } else {
                    (acc.0, acc.1 + 1)
                }
            });
            format!("{}: {}, {}: {}", val_map.up, count.0, val_map.down, count.1)
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
