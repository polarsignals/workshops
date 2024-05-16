package main

import (
	"testing"
)

func TestFindPrimes(t *testing.T) {
	isPrimes := make([]bool, 0, 10+1)

	primes := findPrimes(isPrimes, 10)
	expected := []int{2, 3, 5, 7}
	for i := range expected {
		if primes[i] != expected[i] {
			t.Errorf("expected %d but got %d", expected[i], primes[i])
		}
	}
}

func BenchmarkFindPrimes(b *testing.B) {
	limit := 1_000_000
	isPrimes := make([]bool, 0, limit+1)

	for i := 0; i < b.N; i++ {
		findPrimes(isPrimes, limit)
	}
}
