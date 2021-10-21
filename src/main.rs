use std::time::Instant;
use std::io::{self, BufRead, Write};

struct Interval {
    work: u64,
    rest: u64,
}

impl std::default::Default for Interval {
    fn default() -> Self {
        Interval {
            work: 1, // TODO change to minutes after testing
            rest: 1, // TODO change to minutes after testing
        }
    }
}

impl Interval {
    fn work_set(&self) {
        let now = Instant::now();
        let work_sec = self.work * 60;
        while now.elapsed().as_secs() <= work_sec {
            let sec = work_sec - now.elapsed().as_secs();
            let min = sec / 60;
            print!("\r{}:{:0>2} min of work remaining", min, (sec % 60));
            std::io::stdout().flush().unwrap();
        }
        println!("\nWork set finished\n");

        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle.read_line(&mut buffer).unwrap();
        println!("stdin: {}", buffer);
    }
    fn rest_set(&self) {
        let now = Instant::now();
        let work_sec = self.rest * 60;
        while now.elapsed().as_secs() <= work_sec {
            let sec = work_sec - now.elapsed().as_secs();
            let min = sec / 60;
            print!("\r{}:{:0>2} min of rest remaining", min, (sec % 60));
            std::io::stdout().flush().unwrap();
        }
        println!("\nRest set finished\n");
    }
}

fn main() {
    let interval = Interval::default();
    let now = Instant::now();
    println!("Starting default of {}min work and {}min rest sets", interval.work, interval.rest);
    interval.work_set();
    println!("Starting break of 1");
    interval.rest_set();
    println!("Took {} seconds.", now.elapsed().as_secs());
}
