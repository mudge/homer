use std::net::UdpSocket;

use clap::{App, Arg};

fn main() {
    {
        let matches = App::new("Homer")
            .version("0.1")
            .about("DNS-over-HTTPS proxy")
            .arg(Arg::with_name("bind")
                 .help("listen for DNS requests on this bind address and port")
                 .takes_value(true)
                 .short("b")
                 .long("bind")
                 .default_value("127.0.0.1:53"))
            .arg(Arg::with_name("upstream")
                 .help("URL of upstream DNS-over-HTTPS service")
                 .takes_value(true)
                 .short("u")
                 .long("upstream")
                 .default_value("https://1.1.1.1/dns-query"))
            .get_matches();

        let bind_address = matches.value_of("bind").expect("No bind address found");
        let upstream_url = matches.value_of("upstream").expect("No upstream DOH service URL found");

        let socket = UdpSocket::bind(bind_address).expect(&format!("Could not bind to {}, is something already listening on that address?", bind_address));
        let client = reqwest::Client::new();

        println!("Listening for DNS requests on {}...", bind_address);

        loop {
            // DNS messages shouldn't be more than 512 bytes in length.
            let mut buf = [0; 512];

            if let Ok((number_of_bytes, src_addr)) = socket.recv_from(&mut buf) {
                let filled_buf = &mut buf[..number_of_bytes];

                let result = client.post(upstream_url)
                    .header(reqwest::header::CONTENT_TYPE, "application/dns-message")
                    .body(filled_buf.to_vec())
                    .send();

                match result {
                    Ok(mut response) => {
                        let mut reply_buf: Vec<u8> = Vec::new();

                        match response.copy_to(&mut reply_buf) {
                            Ok(_) => {
                                if let Err(e) = socket.send_to(reply_buf.as_slice(), &src_addr) {
                                    println!("error sending response to client: {:?}", e);
                                }
                            }
                            Err(e) => println!("error reading response from {}: {:?}", upstream_url, e)
                        }
                    }
                    Err(e) => println!("error sending DNS request to {}: {:?}", upstream_url, e)
                }
            }
        }
    }
}
