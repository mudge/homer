use std::net::{SocketAddr, UdpSocket};

use crate::error::{Error, Result};

/// UdpServer represents an infinite series of requests over UDP.
///
/// It implements the `Iterator` trait, yielding successive `Request`s as they are received by the
/// server.
#[derive(Clone, Copy, Debug)]
pub struct UdpServer<'a> {
    /// The underlying UDP socket.
    socket: &'a UdpSocket,
}

/// A UDP request
#[derive(Clone, Debug, PartialEq)]
pub struct Request {
    /// The raw bytes of the request.
    pub body: Vec<u8>,
    /// The origin of the request.
    src_addr: SocketAddr,
}

impl<'a> Iterator for UdpServer<'a> {
    type Item = Request;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut buf = [0; 512];

            if let Ok((number_of_bytes, src_addr)) = self.socket.recv_from(&mut buf) {
                let filled_buf = &mut buf[..number_of_bytes];

                return Some(Request {
                    body: filled_buf.to_vec(),
                    src_addr,
                });
            }
        }
    }
}

impl<'a> UdpServer<'a> {
    /// Returns a new `UdpServer` wrapping the given socket.
    pub fn new(socket: &'a UdpSocket) -> UdpServer {
        UdpServer { socket }
    }

    /// Reply to the given request with the given response over the server's socket.
    pub fn reply(&self, request: &Request, response: &[u8]) -> Result<usize> {
        self.socket
            .send_to(response, request.src_addr)
            .map_err(Error::Response)
    }
}
