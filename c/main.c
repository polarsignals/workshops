#include <stdio.h>
#include <math.h>
#include <stdbool.h>

bool is_prime(int n) {
    if (n <= 1) return false;
    // TODO: Not use sqrt but i*i < n to be inline with other languages
    for (int i = 2; i <= sqrt(n); i++) {
        if (n % i == 0) return false;
    }
    return true;
}

void find_primes(int limit) {
    for (int num = 2; num <= limit; num++) {
        if (is_prime(num)) {
            printf("%d\n", num);
        }
    }
}

int main() {
    int limit = 1000000;
    find_primes(limit);
    return 0;
}
