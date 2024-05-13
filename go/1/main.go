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
	primes := []int{}
	for num := 2; num <= limit; num++ {
		if isPrime(num) {
			primes = append(primes, num)
		}
	}
	return primes
}

func isPrime(n int) bool {
	if n <= 1 {
		return false
	}
	for i := 2; i*i <= n; i++ {
		if n%i == 0 {
			return false
		}
	}
	return true
}
