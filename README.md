# slowdorust

Lightweight slowloris (HTTP DoS) implementation in Rust.

> Slowloris is a denial-of-service attack program which allows an attacker to overwhelm a targeted server by opening and maintaining many simultaneous HTTP connections between the attacker and the target. -[Cloudflare](https://www.cloudflare.com/learning/ddos/ddos-attack-tools/slowloris/)

## Installation

Manual

`cargo install --git https://github.com/MJVL/slowlorust`

## Usage

```
USAGE:
    slowlorust [OPTIONS] <IP> <PORT>

ARGS:
    <IP>      The IP address of the webserver
    <PORT>    The port the webserver is running on

OPTIONS:
    -b, --benchmark-delay <BENCHMARK_DELAY>
            How many seconds to wait between each connection benchmark [default: 15]

    -h, --help
            Print help information

    -l, --lower-sleep <LOWER_SLEEP>
            Lower bound of request delay in seconds [default: 0]

    -n, --num-workers <NUM_WORKERS>
            How many worker sockets to open [default: 50]

    -t, --timeout <TIMEOUT>
            How many seconds to wait before the server is "down" [default: 5]

    -u, --upper-sleep <UPPER_SLEEP>
            Upper bound of request delay in seconds [default: 15]

    -v, --verbose
            Log actions of each worker

    -V, --version
            Print version information
```