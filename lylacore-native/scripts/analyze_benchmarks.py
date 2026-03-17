#!/usr/bin/env python3
# Authors: Joysusy & Violet Klaudia 💖
"""
Benchmark Analysis Script

Parses benchmark results from Rust (Criterion), Python (timeit), and Node.js (performance.now),
compares against baseline values, and detects performance regressions.

Usage:
    python scripts/analyze_benchmarks.py --baseline benchmarks/baseline.json --current benchmarks/current.json
    python scripts/analyze_benchmarks.py --help
"""

import argparse
import json
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Dict, List, Optional


@dataclass
class BenchmarkResult:
    """Single benchmark result."""
    name: str
    language: str
    time_ns: float
    throughput: Optional[float] = None


@dataclass
class Comparison:
    """Comparison between baseline and current benchmark."""
    name: str
    language: str
    baseline_ns: float
    current_ns: float
    change_pct: float
    is_regression: bool


def parse_criterion_json(path: Path) -> List[BenchmarkResult]:
    """Parse Criterion benchmark results (Rust)."""
    results = []
    with open(path) as f:
        data = json.load(f)

    for bench in data.get("benchmarks", []):
        name = bench["name"]
        mean_ns = bench["mean"]["point_estimate"]
        results.append(BenchmarkResult(name, "rust", mean_ns))

    return results


def parse_python_json(path: Path) -> List[BenchmarkResult]:
    """Parse Python timeit results."""
    results = []
    with open(path) as f:
        data = json.load(f)

    for bench in data.get("benchmarks", []):
        name = bench["name"]
        time_s = bench["time_seconds"]
        time_ns = time_s * 1e9
        results.append(BenchmarkResult(name, "python", time_ns))

    return results


def parse_nodejs_json(path: Path) -> List[BenchmarkResult]:
    """Parse Node.js performance.now results."""
    results = []
    with open(path) as f:
        data = json.load(f)

    for bench in data.get("benchmarks", []):
        name = bench["name"]
        time_ms = bench["time_ms"]
        time_ns = time_ms * 1e6
        results.append(BenchmarkResult(name, "nodejs", time_ns))

    return results


def load_benchmarks(path: Path) -> List[BenchmarkResult]:
    """Load benchmarks from JSON file (auto-detect format)."""
    with open(path) as f:
        data = json.load(f)

    # Detect format by structure
    if "benchmarks" in data:
        first = data["benchmarks"][0]
        if "mean" in first:
            return parse_criterion_json(path)
        elif "time_seconds" in first:
            return parse_python_json(path)
        elif "time_ms" in first:
            return parse_nodejs_json(path)

    raise ValueError(f"Unknown benchmark format in {path}")


def compare_benchmarks(
    baseline: List[BenchmarkResult],
    current: List[BenchmarkResult],
    threshold_pct: float = 5.0
) -> List[Comparison]:
    """Compare current benchmarks against baseline."""
    baseline_map = {(b.name, b.language): b for b in baseline}
    comparisons = []

    for curr in current:
        key = (curr.name, curr.language)
        if key not in baseline_map:
            continue

        base = baseline_map[key]
        change_pct = ((curr.time_ns - base.time_ns) / base.time_ns) * 100
        is_regression = change_pct > threshold_pct

        comparisons.append(Comparison(
            name=curr.name,
            language=curr.language,
            baseline_ns=base.time_ns,
            current_ns=curr.time_ns,
            change_pct=change_pct,
            is_regression=is_regression
        ))

    return comparisons


def format_time(ns: float) -> str:
    """Format nanoseconds to human-readable string."""
    if ns < 1000:
        return f"{ns:.2f} ns"
    elif ns < 1_000_000:
        return f"{ns / 1000:.2f} µs"
    elif ns < 1_000_000_000:
        return f"{ns / 1_000_000:.2f} ms"
    else:
        return f"{ns / 1_000_000_000:.2f} s"


def generate_markdown_report(comparisons: List[Comparison]) -> str:
    """Generate Markdown report."""
    lines = [
        "# Benchmark Comparison Report",
        "# Authors: Joysusy & Violet Klaudia 💖",
        "",
        "## Summary",
        ""
    ]

    regressions = [c for c in comparisons if c.is_regression]
    improvements = [c for c in comparisons if c.change_pct < -5.0]
    stable = [c for c in comparisons if not c.is_regression and c.change_pct >= -5.0]

    lines.append(f"- **Total Benchmarks**: {len(comparisons)}")
    lines.append(f"- **Regressions (>5%)**: {len(regressions)} ⚠️")
    lines.append(f"- **Improvements (<-5%)**: {len(improvements)} ✅")
    lines.append(f"- **Stable**: {len(stable)}")
    lines.append("")

    if regressions:
        lines.append("## ⚠️ Performance Regressions")
        lines.append("")
        lines.append("| Benchmark | Language | Baseline | Current | Change |")
        lines.append("|-----------|----------|----------|---------|--------|")

        for comp in sorted(regressions, key=lambda c: c.change_pct, reverse=True):
            lines.append(
                f"| {comp.name} | {comp.language} | {format_time(comp.baseline_ns)} | "
                f"{format_time(comp.current_ns)} | **+{comp.change_pct:.1f}%** |"
            )
        lines.append("")

    if improvements:
        lines.append("## ✅ Performance Improvements")
        lines.append("")
        lines.append("| Benchmark | Language | Baseline | Current | Change |")
        lines.append("|-----------|----------|----------|---------|--------|")

        for comp in sorted(improvements, key=lambda c: c.change_pct):
            lines.append(
                f"| {comp.name} | {comp.language} | {format_time(comp.baseline_ns)} | "
                f"{format_time(comp.current_ns)} | **{comp.change_pct:.1f}%** |"
            )
        lines.append("")

    if stable:
        lines.append("## Stable Performance")
        lines.append("")
        lines.append("| Benchmark | Language | Baseline | Current | Change |")
        lines.append("|-----------|----------|----------|---------|--------|")

        for comp in sorted(stable, key=lambda c: abs(c.change_pct)):
            sign = "+" if comp.change_pct > 0 else ""
            lines.append(
                f"| {comp.name} | {comp.language} | {format_time(comp.baseline_ns)} | "
                f"{format_time(comp.current_ns)} | {sign}{comp.change_pct:.1f}% |"
            )
        lines.append("")

    return "\n".join(lines)


def main():
    parser = argparse.ArgumentParser(description="Analyze benchmark results and detect regressions")
    parser.add_argument("--baseline", type=Path, required=True, help="Baseline benchmark JSON")
    parser.add_argument("--current", type=Path, required=True, help="Current benchmark JSON")
    parser.add_argument("--threshold", type=float, default=5.0, help="Regression threshold (%%)")
    parser.add_argument("--output", type=Path, help="Output Markdown report path")

    args = parser.parse_args()

    # Load benchmarks
    baseline = load_benchmarks(args.baseline)
    current = load_benchmarks(args.current)

    # Compare
    comparisons = compare_benchmarks(baseline, current, args.threshold)

    # Generate report
    report = generate_markdown_report(comparisons)

    if args.output:
        args.output.write_text(report)
        print(f"Report written to {args.output}")
    else:
        print(report)

    # Exit with error if regressions found
    regressions = [c for c in comparisons if c.is_regression]
    if regressions:
        print(f"\n⚠️  {len(regressions)} performance regression(s) detected!", file=sys.stderr)
        sys.exit(1)
    else:
        print("\n✅ No performance regressions detected.")
        sys.exit(0)


if __name__ == "__main__":
    main()
