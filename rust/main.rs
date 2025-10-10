use clap::Parser;
use rust::{find_primes_100000, find_primes_1000000};
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Run in concurrent mode with two parallel tasks
    #[arg(short, long)]
    concurrent: bool,

    /// Run memory allocation comparison example
    #[arg(short, long)]
    memory: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if args.concurrent {
        run_concurrent().await;
    } else if args.memory {
        run_memory_example();
    } else {
        run_single_loop();
    }
}

fn run_single_loop() {
    println!("Starting Rust primes computation (single loop mode)");

    loop {
        let start = Instant::now();

        let count = find_primes_1000000().len();

        let end = Instant::now();
        let duration = end.duration_since(start);
        println!("{:?} primes in {:?} found", count, duration);

        sleep(Duration::from_secs(1));
    }
}

// example profile for this code: https://pprof.me/e783d3cf535192f1bd40f82f8ad9e868
async fn run_concurrent() {
    println!("Starting Rust primes computation (concurrent mode)");

    loop {
        let start = Instant::now();

        // Spawn two concurrent tasks
        let task1 = tokio::spawn(async {
            let task_start = Instant::now();
            let count = find_primes_1000000().len();
            let duration = task_start.elapsed();
            (count, duration)
        });

        let task2 = tokio::spawn(async {
            let task_start = Instant::now();
            let count = find_primes_100000().len();
            let duration = task_start.elapsed();
            (count, duration)
        });

        // Wait for both tasks to complete
        let (result1, result2) = tokio::join!(task1, task2);

        let (count1, duration1) = result1.unwrap();
        let (count2, duration2) = result2.unwrap();

        let total_duration = start.elapsed();

        println!("Concurrent results:");
        println!("  Task 1: {} primes up to 1_000_000 in {:?}", count1, duration1);
        println!("  Task 2: {} primes up to 100_000 in {:?}", count2, duration2);
        println!("  Total time: {:?}", total_duration);

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

// example profile for this code: https://pprof.me/43eeab204f2385da95c64d589ad90c20
fn run_memory_example() {

    println!("Starting memory allocation comparison");
    println!("=========================================\n");

    loop {
        memory_preallocated();
        memory_growing();

        println!("\n-----------------------------------------\n");
        sleep(Duration::from_secs(2));
    }
}

#[inline(never)]
fn memory_preallocated() {
    // Pre-allocated vector with huge capacity
    let large_limit = 100_000_000usize; // 100 million
    println!("Pre-allocated vector (capacity = {} / 100 million):", large_limit);
    let start = Instant::now();
    let mut pre_allocated = Vec::with_capacity(large_limit);

    // Only fill with 100 million elements
    for num in 0..100_000_000 {
        pre_allocated.push(num);
    }

    let duration = start.elapsed();
    println!("  Stored {} elements", pre_allocated.len());
    println!("  Allocated capacity: {}", pre_allocated.capacity());
    println!("  Time: {:?}", duration);
    println!("  Memory wasted: ~{:.2} MB",
             ((pre_allocated.capacity() - pre_allocated.len()) * std::mem::size_of::<usize>()) as f64 / 1_048_576.0);
    println!("  Memory efficiency: {:.4}%\n",
             (pre_allocated.len() as f64 / pre_allocated.capacity() as f64) * 100.0);
}

#[inline(never)]
fn memory_growing() {
    // Growing vector with small initial capacity
    let small_limit = 100_000_000; // 100 million
    println!("Growing vector (initial capacity = 10, target = {} / 100 million):", small_limit);
    let start = Instant::now();
    let mut growing = Vec::with_capacity(10);
    let mut reallocation_count = 0;
    let mut last_capacity = 10;

    for num in 0..small_limit {
        growing.push(num);

        // Track when reallocation happens
        if growing.capacity() != last_capacity {
            reallocation_count += 1;
            if reallocation_count <= 25  {  // Only show first 25 reallocations
                println!("  Reallocation #{}: capacity {} -> {}",
                        reallocation_count, last_capacity, growing.capacity());
            }
            last_capacity = growing.capacity();
        }
    }

    let duration = start.elapsed();
    println!("  Stored {} elements", growing.len());
    println!("  Final capacity: {}", growing.capacity());
    println!("  Total reallocations: {}", reallocation_count);
    println!("  Time: {:?}", duration);
    println!("  Memory used: ~{:.2} MB",
             (growing.capacity() * std::mem::size_of::<usize>()) as f64 / 1_048_576.0);
    println!("  Memory efficiency: {:.2}%",
             (growing.len() as f64 / growing.capacity() as f64) * 100.0);
}
