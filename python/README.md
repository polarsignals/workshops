# Python Workshop

This workshop is designed for software engineers 
who want to learn how to profile their Go applications.
Profiling helps you identify performance bottlenecks in your code and 
writing benchmarks helps optimize its execution.

## Prerequisites

* Basic understanding of Pythong
* Installation for Python 3 (using e.g. venv)
* Linux machine with eBPF enabled

## Run the Parca Agent

### Using Snaps
```
snap install parca-agent --classic --edge
snap set parca-agent metadata-external-labels=machine=<username> 
snap set parca-agent remote-store-bearer-token=<token>
snap start parca-agent
```

Using venv the environment has to be sourced:
```bash
sudo apt install python3.12-venv
python3 -m venv venv
source venv/bin/activate
```

## Run the initial binary

At the same time run the Parca Agent and you'll see the binary being profiled.

```bash
python ./1/main.py
```

In your query in Parca or Polar Signals Cloud filter for `machine="<username>", python="true"` to only see your Python apps.

## Run the tests

```bash
python -m unittest ./1/main.py
```

## Run the benchmarks

We're going to use the [pytest-benchmark](https://pytest-benchmark.readthedocs.io/en/latest/installation.html) library to run the benchmarks.

```bash
pip install pytest-benchmark # install the benchmarking library
py.test 1/main.py
```

After the first initial benchmark run change the code and improve the implementation.
Run the benchmark once more but store the output in a different file.

```bash
py.test 2/main.py
```

## Run the optimized binary

```bash
py.test 2/main.py
```

In your query in Parca or Polar Signals Cloud filter for `machine="<username>", python="true"` to only see your Python apps.

Success! You've optimized the code and improved its performance!
CPU is only 4,61% of what it was before!
Memory can be further improved.
