use clap::{App, Arg};

/// Args represents the configuration of this run of the proxy.
pub struct Args {
    /// The URL to bind the listener to
    pub bind: String,
    /// The URL to proxy requests to
    pub upstream: String,
}

impl Args {
    /// Returns the `Args` for the current run.
    pub fn parse() -> Args {
        let matches = App::new("Homer")
            .version("0.1.0")
            .about("DNS-over-HTTPS proxy")
            .arg(
                Arg::with_name("bind")
                    .help("listen for DNS requests on this bind address and port")
                    .takes_value(true)
                    .short("b")
                    .long("bind")
                    .default_value("127.0.0.1:53"),
            )
            .arg(
                Arg::with_name("upstream")
                    .help("URL of upstream DNS-over-HTTPS service")
                    .takes_value(true)
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
            upstream: matches
                .value_of("upstream")
                .expect("No upstream DNS-over-HTTPS URL found")
                .to_string(),
        }
    }
}
