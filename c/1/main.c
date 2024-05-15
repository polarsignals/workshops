#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <unistd.h>

#define MAX_SIZE 1000000 // 1 million, adjust this value as needed

bool is_prime(int n) {
    if (n <= 1) return false;
    for (int i = 2; i*i <= n; i++){
        if (n%i == 0) {
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
