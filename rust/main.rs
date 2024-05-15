use rust::find_primes2;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

fn main() {
    println!("Starting Rust primes computation");

    let limit = 1_000_000;
    loop {
        let start = Instant::now();

        let count = find_primes2(limit).len();

        let end = Instant::now();
        let duration = end.duration_since(start);
        println!("{:?} primes in {:?} found", count, duration);

        sleep(Duration::from_secs(1));
    }
}
