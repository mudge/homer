use std::net::UdpSocket;

use crate::args::Args;
use crate::udp_server::UdpServer;

mod args;
mod error;
mod udp_server;
mod upstream;

fn main() {
    let args = Args::parse();

    let socket = UdpSocket::bind(&args.bind)
        .expect("could not bind to bind address, is something already listening?");

    println!("Listening for DNS requests on {}...", &args.bind);

    let server = UdpServer::new(&socket);

    let client = reqwest::Client::new();
    let upstreams = args.upstreams(&client);

    for request in server {
        for upstream in upstreams.iter() {
            if let Err(e) = upstream
                .send(&request)
                .map(|response| server.reply(&request, response.as_slice()))
            {
                eprintln!("error during DNS request: {:?}", e);
                continue;
            }

            break;
        }
    }
}
