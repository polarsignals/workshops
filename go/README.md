# Go Workshop

This workshop is designed for software engineers 
who want to learn how to profile their Go applications.
Profiling helps you identify performance bottlenecks in your code and 
writing benchmarks helps optimize its execution.

## Prerequisites

* Basic understanding of Go
* Installation for Go 1.20 or later
* Linux machine with eBPF enabled

## Run the binaries

At the same time run the Parca Agent and you'll see the binary being profiled.

```bash
go build && ./go # or
go run main.go
```

## Run the tests

```bash
go test
go test -run=TestFindPrimes # being specific
```

## Run the benchmarks

```bash
go test -bench=. -count=10 -benchmem -cpuprofile=cpu.out -memprofile=mem.out | tee before
```

After the first initial benchmark run change the code and improve the implementation.
Run the benchmark once more but store the output in a different file.

```bash
go test -bench=. -count=10 -benchmem -cpuprofile=cpu.out -memprofile=mem.out | tee after
```

### Compare the benchmarks

Using [benchstat](https://pkg.go.dev/golang.org/x/perf/cmd/benchstat) it's possible to compare both benchmarks and see the improvements.

Example:
```
name           old time/op    new time/op    delta
FindPrimes-10    49.3ms ± 0%     2.3ms ± 4%  -95.39%  (p=0.000 n=9+10)

name           old alloc/op   new alloc/op   delta
FindPrimes-10    3.22MB ± 0%    4.22MB ± 0%  +31.32%  (p=0.000 n=9+10)

name           old allocs/op  new allocs/op  delta
FindPrimes-10      27.0 ± 0%      28.0 ± 0%   +3.70%  (p=0.000 n=10+10)
```

Success! You've optimized the code and improved its performance!
CPU is only 4,61% of what it was before!
Memory can be further improved.