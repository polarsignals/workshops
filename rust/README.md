# Rust Workshop

This workshop is designed for software engineers 
who want to learn how to profile their Rust applications.
Profiling helps you identify performance bottlenecks in your code and 
writing benchmarks helps optimize its execution.

## Prerequisites

* Basic understanding of Rust
* Installation for Rust
* Linux machine with eBPF enabled

## Run the Parca Agent

### Using Snaps
```
snap install parca-agent --classic --edge
snap set parca-agent metadata-external-labels=machine=<username> 
snap set parca-agent remote-store-bearer-token=<token>
snap start parca-agent
```

## Run the binaries

At the same time run the Parca Agent and you'll see the binary being profiled.

```bash
sudo snap install rustup --classic
rustup default stable
cargo run --release # The --release flag is important for performance!
```

## Run the tests

```bash
cargo test
```

## Run the benchmarks

```bash
cargo bench --bench primes -- --sample-size 10
```

After the first initial benchmark run change the code and improve the implementation.
Run the benchmark once more but store the output in a different file.

```bash
cargo bench --bench primes -- --sample-size 10
```

Success! You've optimized the code and improved its performance!
CPU time spent went from 88ms to 1.7ms. 🎉
