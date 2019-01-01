use clap::{App, Arg, crate_version};

use crate::upstream::Upstream;
use reqwest::Client;

/// Args represents the configuration passed on the command-line.
pub struct Args {
    /// The URL to bind the UDP server to.
    pub bind: String,
    /// The URLs to proxy requests to.
    pub upstream_urls: Vec<String>,
}

impl Args {
    /// Returns the Args for the current run.
    pub fn parse() -> Args {
        let matches = App::new("Homer")
            .version(crate_version!())
            .about("DNS-over-HTTPS proxy")
            .arg(
                Arg::with_name("bind")
                    .help("listen for DNS requests on this address and port")
                    .takes_value(true)
                    .short("b")
                    .long("bind")
                    .default_value("127.0.0.1:53"),
            )
            .arg(
                Arg::with_name("upstream")
                    .help("URL(s) of upstream DNS-over-HTTPS service")
                    .takes_value(true)
                    .multiple(true)
                    .short("u")
                    .long("upstream")
                    .default_value("https://1.1.1.1/dns-query"),
            )
            .get_matches();

        Args {
            bind: matches
                .value_of("bind")
                .expect("No bind address found")
                .to_string(),
            upstream_urls: matches
                .values_of("upstream")
                .expect("No upstream DNS-over-HTTPS URLs found")
                .map(|str| str.to_string())
                .collect(),
        }
    }

    /// Return a vector of Upstreams with the given Client.
    pub fn upstreams<'a>(&'a self, client: &'a Client) -> Vec<Upstream<'a>> {
        self.upstream_urls
            .iter()
            .map(|url| Upstream::new(client, &url))
            .collect()
    }
}
