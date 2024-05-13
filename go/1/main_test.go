package main

import (
	"testing"
)

func TestFindPrimes(t *testing.T) {
	primes := findPrimes(10)
	expected := []int{2, 3, 5, 7}
	for i := range expected {
		if primes[i] != expected[i] {
			t.Errorf("expected %d but got %d", expected[i], primes[i])
		}
	}
}

func BenchmarkFindPrimes(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findPrimes(1_000_000)
	}
}
