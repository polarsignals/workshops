package main

import (
	"log"
	"time"
)

func main() {
	for {
		start := time.Now()
		found := findPrimes(10_000_000)
		log.Printf("%d primes in %s found", len(found), time.Since(start))
		time.Sleep(1 * time.Second)
	}
}

func findPrimes(limit int) []int {
	isPrime := make([]bool, limit+1)
	for i := range isPrime {
		isPrime[i] = true
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
	return primes
}
