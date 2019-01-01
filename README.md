# Homer [![Build Status](https://travis-ci.org/mudge/homer.svg?branch=master)](https://travis-ci.org/mudge/homer)

A lightweight DNS-over-HTTPS ("DOH") proxy written in Rust.

Homer is a lightweight proxy that will securely forward any requests to a DNS-over-HTTPS resolver such as [Cloudflare](https://developers.cloudflare.com/1.1.1.1/dns-over-https/).

**Current version:** 0.2.0  
**Supported Rust version:** 1.31

## Install

Download the latest binary for your architecture from the [releases page](https://github.com/mudge/homer/releases).

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

### Running on a Pi-Hole

To use Homer to encrypt your DNS requests on a Pi-Hole, download and install the latest [release](https://github.com/mudge/homer/releases):

```console
pi@raspberrypi:~ $ wget https://github.com/mudge/homer/releases/download/v0.2.0/homer-v0.2.0-arm-unknown-linux-gnueabihf.tar.gz
pi@raspberrypi:~ $ tar xzf homer-v0.2.0-arm-unknown-linux-gnueabihf.tar.gz
pi@raspberrypi:~ $ sudo mv homer /usr/local/bin/
```

You can confirm Homer is working properly by asking for the current version:

```console
pi@raspberrypi:~ $ homer --version
Homer 0.2.0
```

You can then configure Homer to run as a Systemd service that listens on port 5053 and forwards requests to [Cloudflare's DNS-over-HTTPS resolvers](https://developers.cloudflare.com/1.1.1.1/dns-over-https/).

First, create a system user for Homer:

```console
pi@raspberrypi:~ $ sudo adduser --system --no-create-home homer
```

Then write out a Systemd unit file:

```console
pi@raspberrypi:~ $ sudo tee /lib/systemd/system/homer.service <<EOF
[Unit]
Description=Homer
After=syslog.target network-online.target

[Service]
Type=simple
User=homer
ExecStart=/usr/local/bin/homer -b 127.0.0.1:5053 -u https://1.1.1.1/dns-query -u https://1.0.0.1/dns-query
Restart=on-failure
RestartSec=10
KillMode=process

[Install]
WantedBy=multi-user.target
EOF
```

You can now start up Homer and check it is running:

```console
pi@raspberrypi:~ $ sudo systemctl enable homer
pi@raspberrypi:~ $ sudo systemctl start homer
pi@raspberrypi:~ $ sudo systemctl status homer
```

Finally, you can change your Pi-Hole configuration to use `127.0.0.1#5053` as its sole upstream DNS server and confirm your requests are now secure by using [Cloudflare's connection information page](https://1.1.1.1/help).

## References

* https://developers.cloudflare.com/1.1.1.1/dns-over-https/

## License

Copyright © 2018 Paul Mucur

Distributed under the MIT License.
