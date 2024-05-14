import time
import unittest

def findPrimesSlower(limit = 10_000):
	primes = []
	for i in range(2, limit):
		if isPrime(i):
			primes.append(i)
	return primes

def isPrime(n):
	if n <= 1:
		return False
	for i in range(2, n):
		if i * i > n:
			break
		if n%i == 0:
			return False
	return True

# improvment using sieve_of_eratosthenes
def findPrimes(limit = 10_000):
	is_prime = [True] * (limit + 1)
	p = 2
	while (p * p <= limit):
		if (is_prime[p] == True):
			for i in range(p * p, limit + 1, p):
				is_prime[i] = False
		p += 1
	primes = [p for p in range(2, limit + 1) if is_prime[p]]
	return primes

if __name__ == '__main__':
	while True:
		start = time.time()
		count = len(findPrimes(1_000_000))
		end = time.time()
		print("%d primes in %fs found" % (count, end - start))
		time.sleep(1)

class PrimeTest(unittest.TestCase):
	def test_primes(self):
		self.assertEqual([2, 3, 5, 7], findPrimesSlower(10))

class PrimeTestFaster(unittest.TestCase):
	def test_primes(self):
		self.assertEqual([2, 3, 5, 7], findPrimes(10))

# Benchmark both implementation
# From the python directory run the following command
# py.test 2/main.py

def test_primes(benchmark):
	benchmark(findPrimesSlower)

def test_primes_faster(benchmark):
	benchmark(findPrimes)
