use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust::{find_primes, find_primes2};

fn criterion_benchmark(c: &mut Criterion) {
    const SIZE: usize = 1_000_000;
    c.bench_function("naive 10M", |b| b.iter(|| find_primes(black_box(SIZE))));
    c.bench_function("better 10M", |b| b.iter(|| find_primes2(black_box(SIZE))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);