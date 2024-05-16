package main

import (
	"log"
	"time"
)

func main() {
	limit := 10_000_000
	isPrime := make([]bool, 0, limit+1)

	for {
		start := time.Now()
		found := findPrimes(isPrime, limit)
		log.Printf("%d primes in %s found", len(found), time.Since(start))
		time.Sleep(1 * time.Second)
	}
}

func findPrimes(isPrime []bool, limit int) []int {
	for _ = range limit + 1 {
		isPrime = append(isPrime, true)
	}
	for p := 2; p*p <= limit; p++ {
		if isPrime[p] {
			for i := p * p; i <= limit; i += p {
				isPrime[i] = false
			}
		}
	}
	primes := []int{}
	for p := 2; p <= limit; p++ {
		if isPrime[p] {
			primes = append(primes, p)
		}
	}

	isPrime = isPrime[:0]

	return primes
}
