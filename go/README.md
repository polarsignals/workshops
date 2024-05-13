# Go Workshop

This workshop is designed for software engineers 
who want to learn how to profile their Go applications.
Profiling helps you identify performance bottlenecks in your code and 
writing benchmarks helps optimize its execution.

## Prerequisites

* Basic understanding of Go
* Installation for Go 1.20 or later
* Linux machine with eBPF enabled

## Run the Parca Agent

### Using Snaps
```
snap install parca-agent --classic --edge
snap set parca-agent metadata-external-labels=username=<username> 
snap set parca-agent remote-store-bearer-token=<token>
snap start parca-agent
```

## Run the initial binary

At the same time run the Parca Agent and you'll see the binary being profiled.

```bash
go build -v -o bin/1 ./1 && ./bin/1
# or
go run ./1
```

In your query in Parca or Polar Signals Cloud filter for `comm="1"` to only see this binary.

## Run the tests

```bash
go test ./1
go test -run=TestFindPrimes ./1 # being specific
```

## Run the benchmarks

```bash
go test -bench=. -count=10 -benchmem -cpuprofile=1/cpu.out -memprofile=1/mem.out ./1 | tee 1/benchmark.txt
```

After the first initial benchmark run change the code and improve the implementation.
Run the benchmark once more but store the output in a different file.

```bash
go test -bench=. -count=10 -benchmem -cpuprofile=2/cpu.out -memprofile=2/mem.out ./2 | tee 2/benchmark.txt
```

## Run the optimized binary

```bash
go build -v -o bin/2 ./2 && ./bin/2
# or
go run ./2
```

In your query in Parca or Polar Signals Cloud filter for `comm="2"` to only see this binary.

### Compare the benchmarks

Using [benchstat](https://pkg.go.dev/golang.org/x/perf/cmd/benchstat) it's possible to compare both benchmarks and see the improvements.

_Note: Unfortunately, we need to update the third line in each benchmark.txt to be the same._  
_Remove the trailing `/1` and `/2` so you're left with: `pkg: github.com/polarsignals/workshops/go`._

Run `benchstat ./1/benchmark.txt ./2/benchmark.txt`:
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