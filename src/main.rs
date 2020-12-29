use std::result::Result;

use structopt::StructOpt;
use tokio;
use reqwest::{StatusCode, get};

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

async fn map_statuses(urls: &mut dyn Iterator<Item = String>) -> Vec<bool> {
    let results = Vec::new();
    for url in urls {
        // Spawn a thread for each request
        tokio::task::spawn(async {
            // Send get request, check for reqwest error for bad url and warn if so
            let res = get(&url).await;
            results.push((url, res?.status()));
        });
    }
    results
}

#[tokio::main]
async fn main() -> Result<(), &'static str> {
    let args = Opt::from_args();

    if args.urls.is_empty() {
        return Err("Please pass in valid urls you would like to monitor")
    } else {

        Ok(())
    }
}
