# Homer [![Build Status](https://travis-ci.org/mudge/homer.svg?branch=master)](https://travis-ci.org/mudge/homer)

A minimal Cloudflare DNS-over-HTTPS ("DOH") proxy written in Rust.

**Current version:** 0.2.0  
**Supported Rust version:** 1.31

## Install

Download the latest binary from the [releases page](https://github.com/mudge/homer/releases).

## Usage

```
USAGE:
    homer [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --bind <bind>               listen for DNS requests on this address and port [default: 127.0.0.1:53]
    -u, --upstream <upstream>...    URL(s) of upstream DNS-over-HTTPS service [default: https://1.1.1.1/dns-query]
```

## References

* https://developers.cloudflare.com/1.1.1.1/dns-over-https/

## License

Copyright © 2018 Paul Mucur

Distributed under the MIT License.
