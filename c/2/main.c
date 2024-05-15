#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <unistd.h>

#define MAX_SIZE 1000000 // 1 million, adjust this value as needed

// improvement using sieve_of_eratosthenes
int find_primes(int limit) {
  // Create an array to mark prime numbers (1 indicates not prime)
  bool is_prime[limit + 1];

  // Initialize all elements as prime (except 0 and 1)
  for (int i = 0; i <= limit; i++) {
    is_prime[i] = true;
  }

  is_prime[0] = is_prime[1] = false;

  // Sieve logic: Mark multiples of primes as non-prime
  for (int i = 2; i * i <= limit; i++) {
    if (is_prime[i]) {
      // Mark all multiples of i as non-prime (except i itself)
      for (int j = i * i; j <= limit; j += i) {
        is_prime[j] = false;
      }
    }
  }

  // Print prime numbers
  int count;
  for (int i = 2; i <= limit; i++) {
    if (is_prime[i]) {
        count++;
    }
  }

  return count;
}

int main() {
    printf("Starting C primes computation");

    while (true) {
        clock_t start = clock();

        int limit = MAX_SIZE;
        int count = find_primes(limit);

        clock_t end = clock();
        printf("%d primes in %f seconds found\n", count, (double)(end - start) / CLOCKS_PER_SEC);
        sleep(1);
    }
    return 0;
}
