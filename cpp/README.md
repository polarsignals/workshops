# C++ Workshop

This workshop is designed for software engineers 
who want to learn how to profile their C++ applications.
Profiling helps you identify performance bottlenecks in your code and 
writing benchmarks helps optimize its execution.

_Note: This workshop contains two example binaries, but no tests or benchmarks. Profiling still works. For tests and benchmarks refer to either Rust or Go._

## Prerequisites

* Basic understanding of C++
* Installation for g++ (apt install g++)
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
g++ -std=c++11 1/main.cpp -o ./bin/1.cpp
./bin/1
# or
g++ -std=c++11 1/main.cpp -o ./bin/1.cpp && ./bin/1.cpp
```

## Run the tests and benchmarks

In real-world applications, you would write tests and benchmarks to ensure the correctness and performance of your code.
Then use the benchmark to optimize the code further and further.

Once the optimizations are done, the new version gets deployed and is profiled again.

```bash
g++ -std=c++11 2/main.cpp -o ./bin/2.cpp
./bin/2.cpp
# or
g++ -std=c++11 2/main.cpp -o ./bin/2.cpp && ./bin/2.cpp
```

Success! You've optimized the code and improved its performance!
Looking at the durations, we've optimized the program by roughly 10x. 🎉
