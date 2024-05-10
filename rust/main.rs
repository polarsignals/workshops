use std::f64;

fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    let limit = (n as f64).sqrt() as i32;
    for i in 2..=limit {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn find_primes(limit: i32) {
    for num in 2..=limit {
        if is_prime(num) {
            println!("{}", num);
        }
    }
}

fn main() {
    // TODO: Wrap with endless loop and 1s sleep
    let limit = 1_000_000;
    find_primes(limit);
}
