import time
import unittest

def findPrimes(limit = 10_000):
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

# # improvment using sieve_of_eratosthenes
# def findPrimesFaster(limit = 10_000):
# 	is_prime = [True] * (limit + 1)
# 	p = 2
# 	while (p * p <= limit):
# 		if (is_prime[p] == True):
# 			for i in range(p * p, limit + 1, p):
# 				is_prime[i] = False
# 		p += 1
# 	primes = [p for p in range(2, limit + 1) if is_prime[p]]
# 	return primes

if __name__ == '__main__':
	while True:
		print(len(findPrimes(10_000)))
		# print(findPrimesFaster(10))
		time.sleep(1)

# class PrimeTest(unittest.TestCase):
# 	def test_primes(self):
# 		self.assertEqual([2, 3, 5, 7], findPrimes(10))
#
# if __name__ == '__main__':
# 	unittest.main()

# Benchmark both implementation
# From the python directory run the following command
# py.test main.py

# def test_my_stuff(benchmark):
# 	benchmark(findPrimes)
#
# def test_my_stuff_faster(benchmark):
# 	benchmark(findPrimesFaster)
