use std::thread::sleep;
use std::time::Duration;

fn main() {
    loop {
        let limit = 10_000_000;
        println!("{:?}", find_primes(limit).len());
        sleep(Duration::from_secs(1));
    }
}

fn find_primes(limit: usize) -> Vec<usize> {
    let mut primes = Vec::new();
    for num in 2..=limit {
        if is_prime(num) {
            primes.push(num);
        }
    }
    return primes;
}

fn find_primes2(limit: usize) -> Vec<usize> {
    let mut primes = vec![true; limit];

    for p in (2..).take_while(|&p| p * p <= limit) {
        if primes[p] {
            for i in (p * p..limit).step_by(p) {
                primes[i] = false;
            }
        }
    }
    return (2..limit).filter(|&i| primes[i]).collect();
}

fn is_prime(n: usize) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_primes() {
        let result = find_primes(10);
        assert_eq!(result, [2, 3, 5, 7]);
    }

    #[test]
    fn test_find_primes2() {
        let result = find_primes2(10);
        assert_eq!(result, [2, 3, 5, 7]);
    }
}
