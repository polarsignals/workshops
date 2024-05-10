use std::thread::sleep;
use std::time::Duration;

fn main() {
    loop {
        let limit = 100;
        println!("{:?}", find_primes(limit));
        sleep(Duration::from_secs(1));
    }
}

fn find_primes(limit: i32) -> Vec<i32> {
    let mut primes = Vec::new();
    for num in 2..=limit {
        if is_prime(num) {
            primes.push(num);
        }
    }
    return primes;
}

fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=n {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            return false;
        }
    }
    true
}

// TODO: Add the improved version using sieve_of_eratosthenes
