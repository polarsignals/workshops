import time
import unittest


def find_primes_slower(limit = 10_000):
	primes = []
	for i in range(2, limit):
		if is_prime(i):
			primes.append(i)
	return primes


def is_prime(n):
	if n <= 1:
		return False
	for i in range(2, n):
		if i * i > n:
			break
		if n % i == 0:
			return False
	return True


def find_primes(limit = 10_000):
	"""improvment using sieve_of_eratosthenes"""
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
		count = len(find_primes(1_000_000))
		end = time.time()
		print(f"{count} primes in {end - start}s found")
		time.sleep(1)


class PrimeTest(unittest.TestCase):
	def test_primes(self):
		self.assertEqual([2, 3, 5, 7], find_primes_slower(10))


class PrimeTestFaster(unittest.TestCase):
	def test_primes(self):
		self.assertEqual([2, 3, 5, 7], find_primes(10))


# Benchmark both implementation
# From the python directory run the following command
# py.test 2/main.py


def test_primes(benchmark):
	benchmark(find_primes_slower)


def test_primes_faster(benchmark):
	benchmark(find_primes)
