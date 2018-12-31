# Homer

A minimal Cloudflare DNS-over-HTTPS ("DOH") proxy written in Rust.

**Current version:** 0.1.0  
**Supported Rust version:** 1.31

## Usage

```
USAGE:
    homer [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --bind <bind>            listen for DNS requests on this bind address and port [default: 127.0.0.1:53]
    -u, --upstream <upstream>    URL of upstream DNS-over-HTTPS service [default: https://1.1.1.1/dns-query]
```

* https://developers.cloudflare.com/1.1.1.1/dns-over-https/

## License

Copyright © 2018 Paul Mucur

Distributed under the MIT License.
