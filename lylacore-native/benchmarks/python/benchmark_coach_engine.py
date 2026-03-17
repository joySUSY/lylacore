#!/usr/bin/env python3
# Authors: Joysusy & Violet Klaudia 💖
"""
COACH Engine Performance Benchmarks

Comprehensive benchmarking suite for coach-engine Python bindings.
Measures pattern learning, style application, and analysis performance.
"""

import sys
import timeit
from pathlib import Path

# Add parent directory to path to import lylacore
sys.path.insert(0, str(Path(__file__).parent.parent.parent / "crates" / "pyo3-bindings"))

try:
    from lylacore import coach_engine
except ImportError:
    print("Error: lylacore module not found. Build with: maturin develop")
    sys.exit(1)


def benchmark_pattern_learning():
    """Benchmark pattern learning operations."""
    print("\n=== Pattern Learning Benchmarks ===\n")

    user_message = "Hey! How's it going? 😊"
    agent_response = "I'm doing great, thanks for asking!"
    context = {"language": "en", "topic": "greeting"}

    # New pattern
    def bench_new():
        coach_engine.learn_pattern(user_message, agent_response, context)

    iterations = 10000
    time_taken = timeit.timeit(bench_new, number=iterations) / iterations
    print(f"Learn pattern (new)    | {time_taken * 1e6:8.2f} µs")

    # Update existing
    existing_style = coach_engine.learn_pattern(user_message, agent_response, context)

    def bench_update():
        coach_engine.learn_pattern(user_message, agent_response, context, existing_style)

    time_taken = timeit.timeit(bench_update, number=iterations) / iterations
    print(f"Learn pattern (update) | {time_taken * 1e6:8.2f} µs")


def benchmark_style_application():
    """Benchmark style application."""
    print("\n=== Style Application Benchmarks ===\n")

    user_message = "Hey! How's it going? 😊"
    agent_response = "I'm doing great, thanks for asking!"
    context = {"language": "en", "topic": "greeting"}

    style = coach_engine.learn_pattern(user_message, agent_response, context)
    message = "This is a test message for style application"

    def bench():
        coach_engine.apply_style(message, style)

    iterations = 100000
    time_taken = timeit.timeit(bench, number=iterations) / iterations
    print(f"Apply style | {time_taken * 1e9:8.2f} ns")


def benchmark_pattern_merging():
    """Benchmark pattern merging."""
    print("\n=== Pattern Merging Benchmarks ===\n")

    context = {"language": "en", "topic": "general"}

    # Create patterns
    patterns = [
        coach_engine.learn_pattern(f"Message {i}", f"Response {i}", context)
        for i in range(10)
    ]

    for count in [2, 5, 10]:
        def bench():
            coach_engine.merge_patterns(patterns[:count])

        iterations = 10000
        time_taken = timeit.timeit(bench, number=iterations) / iterations
        print(f"Merge {count:2} patterns | {time_taken * 1e6:8.2f} µs")


def benchmark_style_analysis():
    """Benchmark style analysis."""
    print("\n=== Style Analysis Benchmarks ===\n")

    messages = [
        "Hey! How's it going? 😊",
        "What's up?",
        "How are you doing today?",
        "Hope you're having a great day!",
        "Thanks so much for your help!",
    ]

    def bench():
        coach_engine.analyze_style(messages)

    iterations = 10000
    time_taken = timeit.timeit(bench, number=iterations) / iterations
    print(f"Analyze style | {time_taken * 1e6:8.2f} µs")


def main():
    """Run all benchmarks."""
    print("=" * 60)
    print("COACH Engine Python Bindings - Performance Benchmarks")
    print("=" * 60)

    benchmark_pattern_learning()
    benchmark_style_application()
    benchmark_pattern_merging()
    benchmark_style_analysis()

    print("\n" + "=" * 60)
    print("Benchmarks Complete")
    print("=" * 60)


if __name__ == "__main__":
    main()
