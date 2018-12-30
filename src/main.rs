use std::net::UdpSocket;

fn main() {
    {
        let socket = UdpSocket::bind("127.0.0.1:5053").expect("Could not bind to 5053, is something else listening?");
        let client = reqwest::Client::new();

        println!("Listening for DNS requests on 127.0.0.1:5053...");

        loop {
            // DNS messages shouldn't be more than 512 bytes in length.
            let mut buf = [0; 512];

            if let Ok((number_of_bytes, src_addr)) = socket.recv_from(&mut buf) {
                let filled_buf = &mut buf[..number_of_bytes];

                let result = client.post("https://1.1.1.1/dns-query")
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
                            Err(e) => println!("error reading response from Cloudflare: {:?}", e)
                        }
                    }
                    Err(e) => println!("error sending DNS request to Cloudflare: {:?}", e)
                }
            }
        }
    }
}
