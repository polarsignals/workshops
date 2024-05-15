import time
import unittest


def find_primes(limit = 10_000):
	primes = []
	for i in range(2, limit):
		if is_rime(i):
			primes.append(i)
	return primes


def is_prime(n):
	if n <= 1:
		return False
	for i in range(2, n):
		if i * i > n:
			break
		if n%i == 0:
			return False
	return True


if __name__ == '__main__':
	while True:
		start = time.time()
		count = len(find_primes(1_000_000))
		end = time.time()
		print(f"{count} primes in {end - start}s found")
		time.sleep(1)


class PrimeTest(unittest.TestCase):
	def test_primes(self):
		self.assertEqual([2, 3, 5, 7], find_primes(10))


if __name__ == '__unittest__':
	unittest.main()


def test_my_stuff(benchmark):
	benchmark(find_primes)
