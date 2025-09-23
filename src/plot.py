import re
import matplotlib.pyplot as plt
import numpy as np

def parse_benchmark_output(text):
    """Parse the benchmark output and extract data."""
    data = []

    # Pattern to match benchmark results
    pattern = r'Benchmarking powerset construction with (\d+) states.*?from\(\)\s*:\s*([\d.]+)([µm]?)s.*?from_old\(\)\s*:\s*([\d.]+)([µm]?)s'

    matches = re.findall(pattern, text, re.DOTALL)

    for match in matches:
        nfa_states = int(match[0])

        # Parse from() time
        from_time = float(match[1])
        from_unit = match[2]
        if from_unit == 'µ':
            from_time *= 1e-6  # microseconds to seconds
        elif from_unit == 'm':
            from_time *= 1e-3  # milliseconds to seconds

        # Parse from_old() time
        from_old_time = float(match[3])
        from_old_unit = match[4]
        if from_old_unit == 'µ':
            from_old_time *= 1e-6  # microseconds to seconds
        elif from_old_unit == 'm':
            from_old_time *= 1e-3  # milliseconds to seconds

        data.append({
            'nfa_states': nfa_states,
            'from_time': from_time,
            'from_old_time': from_old_time
        })

    return data

def plot_benchmarks(data):
    """Create both linear and logarithmic plots."""
    nfa_states = [d['nfa_states'] for d in data]
    from_times = [d['from_time'] for d in data]
    from_old_times = [d['from_old_time'] for d in data]

    # Create figure with 2 subplots
    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(15, 6))

    # Linear scale plot
    ax1.plot(nfa_states, from_times, 'b-o', label='from()', markersize=4)
    ax1.plot(nfa_states, from_old_times, 'r-s', label='from_old()', markersize=4)
    ax1.set_xlabel('NFA States')
    ax1.set_ylabel('Time (seconds)')
    ax1.set_title('Powerset Construction Performance (Linear Scale)')
    ax1.legend()
    ax1.grid(True, alpha=0.3)

    # Logarithmic scale plot
    ax2.loglog(nfa_states, from_times, 'b-o', label='from()', markersize=4)
    ax2.loglog(nfa_states, from_old_times, 'r-s', label='from_old()', markersize=4)
    ax2.set_xlabel('NFA States')
    ax2.set_ylabel('Time (seconds)')
    ax2.set_title('Powerset Construction Performance (Log Scale)')
    ax2.legend()
    ax2.grid(True, alpha=0.3)

    plt.tight_layout()
    return fig

# Benchmark data (paste your output here)
benchmark_text = """
============================================================
POWERSET CONSTRUCTION BENCHMARK
============================================================
Benchmarking powerset construction with 3 states
Generated pathological NFA with 4 states and 9 transitions
Results:
  from()     : 174.027µs
  from_old() : 170.972µs
  Speedup    : 0.98x
  DFA states : 8

Benchmarking powerset construction with 4 states
Generated pathological NFA with 5 states and 11 transitions
Results:
  from()     : 339.659µs
  from_old() : 377.851µs
  Speedup    : 1.11x
  DFA states : 16

Benchmarking powerset construction with 5 states
Generated pathological NFA with 6 states and 13 transitions
Results:
  from()     : 689.837µs
  from_old() : 840.02µs
  Speedup    : 1.22x
  DFA states : 32

Benchmarking powerset construction with 6 states
Generated pathological NFA with 7 states and 15 transitions
Results:
  from()     : 1.739753ms
  from_old() : 2.006323ms
  Speedup    : 1.15x
  DFA states : 64

Benchmarking powerset construction with 7 states
Generated pathological NFA with 8 states and 17 transitions
Results:
  from()     : 3.241186ms
  from_old() : 4.038576ms
  Speedup    : 1.25x
  DFA states : 128

Benchmarking powerset construction with 8 states
Generated pathological NFA with 9 states and 19 transitions
Results:
  from()     : 6.935274ms
  from_old() : 8.998515ms
  Speedup    : 1.30x
  DFA states : 256

Benchmarking powerset construction with 9 states
Generated pathological NFA with 10 states and 21 transitions
Results:
  from()     : 14.904003ms
  from_old() : 19.315831ms
  Speedup    : 1.30x
  DFA states : 512

Benchmarking powerset construction with 10 states
Generated pathological NFA with 11 states and 23 transitions
Results:
  from()     : 32.912775ms
  from_old() : 42.664506ms
  Speedup    : 1.30x
  DFA states : 1024

Benchmarking powerset construction with 11 states
Generated pathological NFA with 12 states and 25 transitions
Results:
  from()     : 70.321666ms
  from_old() : 93.801789ms
  Speedup    : 1.33x
  DFA states : 2048

Benchmarking powerset construction with 12 states
Generated pathological NFA with 13 states and 27 transitions
Results:
  from()     : 154.647141ms
  from_old() : 204.672983ms
  Speedup    : 1.32x
  DFA states : 4096

Benchmarking powerset construction with 13 states
Generated pathological NFA with 14 states and 29 transitions
Results:
  from()     : 329.553386ms
  from_old() : 440.904551ms
  Speedup    : 1.34x
  DFA states : 8192

Benchmarking powerset construction with 14 states
Generated pathological NFA with 15 states and 31 transitions
Results:
  from()     : 700.462484ms
  from_old() : 965.379112ms
  Speedup    : 1.38x
  DFA states : 16384

Benchmarking powerset construction with 15 states
Generated pathological NFA with 16 states and 33 transitions
Results:
  from()     : 1.509557299s
  from_old() : 2.059834025s
  Speedup    : 1.36x
  DFA states : 32768

Benchmarking powerset construction with 16 states
Generated pathological NFA with 17 states and 35 transitions
Results:
  from()     : 3.224274984s
  from_old() : 4.440678854s
  Speedup    : 1.38x
  DFA states : 65536

Benchmarking powerset construction with 17 states
Generated pathological NFA with 18 states and 37 transitions
Results:
  from()     : 6.86327364s
  from_old() : 9.615445593s
  Speedup    : 1.40x
  DFA states : 131072

Benchmarking powerset construction with 18 states
Generated pathological NFA with 19 states and 39 transitions
Results:
  from()     : 14.658231822s
  from_old() : 20.784898942s
  Speedup    : 1.42x
  DFA states : 262144

Benchmarking powerset construction with 19 states
Generated pathological NFA with 20 states and 41 transitions
Results:
  from()     : 31.365979861s
  from_old() : 44.69196922s
  Speedup    : 1.42x
  DFA states : 524288

Benchmarking powerset construction with 20 states
Generated pathological NFA with 21 states and 43 transitions
Results:
  from()     : 66.299301546s
  from_old() : 94.541210537s
  Speedup    : 1.43x
  DFA states : 1048576
"""

if __name__ == "__main__":
    # Parse the benchmark data
    data = parse_benchmark_output(benchmark_text)

    # Create and display plots
    fig = plot_benchmarks(data)
    plt.show()

    # Print some statistics
    print(f"Parsed {len(data)} benchmark results")
    print(f"NFA states range: {min(d['nfa_states'] for d in data)} to {max(d['nfa_states'] for d in data)}")

    # Calculate average speedup
    speedups = [d['from_old_time'] / d['from_time'] for d in data]
    avg_speedup = np.mean(speedups)
    print(f"Average speedup: {avg_speedup:.2f}x")