#include <chrono>
#include <iostream>
#include <unistd.h>
#include <vector>

using namespace std;

// improvement using sieve_of_eratosthenes
int find_primes(int limit) {
  // Create a boolean vector to mark prime numbers (true) and non-primes (false)
  vector<bool> is_prime(limit + 1, true);

  // 0 and 1 are not prime
  is_prime[0] = is_prime[1] = false;

  // Iterate through all numbers up to the square root of n
  for (int i = 2; i * i <= limit; i++) {
    if (is_prime[i]) {
      // Mark all multiples of i as non-prime
      for (int j = i * i; j <= limit; j += i) {
        is_prime[j] = false;
      }
    }
  }

  int count;
  for (int i = 2; i <= limit; i++) {
    if (is_prime[i]) {
      count++;
    }
  }

  return count;
}

int main() {
  std::cout << "Starting C++ primes computation" << std::endl;

  while (true) {
    auto start = std::chrono::high_resolution_clock::now();

    int limit = 10000000;
    int count = find_primes(limit);

    auto end = std::chrono::high_resolution_clock::now();
    auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(end - start);

    std::cout << count << " primes in " << duration.count() << " milliseconds found\n";

    sleep(1);
  }

  return 0;
}
