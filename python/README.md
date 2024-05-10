# Python Workshop

This workshop is designed for software engineers 
who want to learn how to profile their Go applications.
Profiling helps you identify performance bottlenecks in your code and 
writing benchmarks helps optimize its execution.

## Prerequisites

* Basic understanding of Pythong
* Installation for Python 3 using venv
* Linux machine with eBPF enabled

```bash
python3 -m venv venv
source venv/bin/activate
```

```bash
pip install pytest-benchmark
```