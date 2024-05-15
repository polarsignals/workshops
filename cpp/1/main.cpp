#include <chrono>
#include <iostream>
#include <unistd.h>

bool is_prime(int n) {
  if (n <= 1) {
    return false;
  }
  for (int i = 2; i * i <= n; i++) {
    if (n % i == 0) {
      return false;
    }
  }
  return true;
}

int find_primes(int limit) {
    int count = 0;
    for (int num = 2; num <= limit; num++) {
        if (is_prime(num)) {
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
