#!/usr/bin/env python3
"""
Script to plot benchmark results from benchmark.csv
Shows the relationship between number of characters and duration in seconds.
"""

import pandas as pd
import matplotlib.pyplot as plt
import numpy as np

def main():
    # Read the CSV file
    df = pd.read_csv('dfa_klenee.csv')

    # Create the plot
    plt.figure(figsize=(12, 8))
    plt.plot(df['number_of_characters'], df['duration_seconds'], 'b-', linewidth=2, alpha=0.7)
    plt.scatter(df['number_of_characters'], df['duration_seconds'], color='red', s=20, alpha=0.6)

    # Customize the plot
    plt.xlabel('Number of Characters', fontsize=12)
    plt.ylabel('Duration (seconds)', fontsize=12)
    plt.title('Benchmark Results: Duration vs Number of Characters', fontsize=14, fontweight='bold')
    plt.grid(True, alpha=0.3)

    # Add some statistics to the plot
    max_duration = df['duration_seconds'].max()
    max_chars = df.loc[df['duration_seconds'].idxmax(), 'number_of_characters']
    plt.annotate(f'Max: {max_duration:.6f}s at {max_chars} chars',
                xy=(max_chars, max_duration),
                xytext=(max_chars-20, max_duration+0.001),
                arrowprops=dict(arrowstyle='->', color='red', alpha=0.7),
                fontsize=10)

    # Fit a polynomial trend line
    z = np.polyfit(df['number_of_characters'], df['duration_seconds'], 2)
    p = np.poly1d(z)
    plt.plot(df['number_of_characters'], p(df['number_of_characters']), 'g--',
             alpha=0.8, linewidth=1, label='Trend')

    plt.legend()
    plt.tight_layout()

    # Save the plot
    plt.savefig('benchmark_plot.png', dpi=300, bbox_inches='tight')
    print("Plot saved as 'benchmark_plot.png'")

    # Show the plot
    plt.show()

    # Print some basic statistics
    print(f"\nBenchmark Statistics:")
    print(f"Total data points: {len(df)}")
    print(f"Character range: {df['number_of_characters'].min()} - {df['number_of_characters'].max()}")
    print(f"Duration range: {df['duration_seconds'].min():.6f}s - {df['duration_seconds'].max():.6f}s")
    print(f"Average duration: {df['duration_seconds'].mean():.6f}s")

if __name__ == "__main__":
    main()
