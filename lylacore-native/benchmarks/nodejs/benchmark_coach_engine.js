// Authors: Joysusy & Violet Klaudia 💖
/**
 * COACH Engine Performance Benchmarks
 *
 * Comprehensive benchmarking suite for coach-engine Node.js bindings.
 * Measures pattern learning, style application, and analysis performance.
 */

const { performance } = require('perf_hooks');

// Import the native addon
let coachEngine;
try {
    coachEngine = require('../../crates/napi-bindings');
} catch (error) {
    console.error('Error: lylacore native addon not found. Build with: npm run build');
    process.exit(1);
}

/**
 * Run a benchmark with adaptive iterations
 */
function benchmark(name, fn, minIterations = 100, targetTime = 1000) {
    // Warm-up
    for (let i = 0; i < 10; i++) {
        fn();
    }

    // Determine iteration count
    const start = performance.now();
    fn();
    const singleTime = performance.now() - start;

    const iterations = Math.max(minIterations, Math.floor(targetTime / singleTime));

    // Actual benchmark
    const benchStart = performance.now();
    for (let i = 0; i < iterations; i++) {
        fn();
    }
    const benchEnd = performance.now();

    const avgTime = (benchEnd - benchStart) / iterations;

    // Format output
    let timeStr;
    if (avgTime < 0.001) {
        timeStr = `${(avgTime * 1e6).toFixed(2)} ns`;
    } else if (avgTime < 1) {
        timeStr = `${(avgTime * 1e3).toFixed(2)} µs`;
    } else {
        timeStr = `${avgTime.toFixed(2)} ms`;
    }

    console.log(`${name.padEnd(30)} | ${timeStr.padStart(12)}`);
}

/**
 * Benchmark pattern learning operations
 */
function benchmarkPatternLearning() {
    console.log('\n=== Pattern Learning Benchmarks ===\n');

    const userMessage = "Hey! How's it going? 😊";
    const agentResponse = "I'm doing great, thanks for asking!";
    const context = { language: 'en', topic: 'greeting' };

    // New pattern
    benchmark('Learn pattern (new)', () => {
        coachEngine.learnPattern(userMessage, agentResponse, context);
    }, 1000);

    // Update existing
    const existingStyle = coachEngine.learnPattern(userMessage, agentResponse, context);

    benchmark('Learn pattern (update)', () => {
        coachEngine.learnPattern(userMessage, agentResponse, context, existingStyle);
    }, 1000);
}

/**
 * Benchmark style application
 */
function benchmarkStyleApplication() {
    console.log('\n=== Style Application Benchmarks ===\n');

    const userMessage = "Hey! How's it going? 😊";
    const agentResponse = "I'm doing great, thanks for asking!";
    const context = { language: 'en', topic: 'greeting' };

    const style = coachEngine.learnPattern(userMessage, agentResponse, context);
    const message = 'This is a test message for style application';

    benchmark('Apply style', () => {
        coachEngine.applyStyle(message, style);
    }, 10000);
}

/**
 * Benchmark pattern merging
 */
function benchmarkPatternMerging() {
    console.log('\n=== Pattern Merging Benchmarks ===\n');

    const context = { language: 'en', topic: 'general' };

    // Create patterns
    const patterns = [];
    for (let i = 0; i < 10; i++) {
        patterns.push(
            coachEngine.learnPattern(`Message ${i}`, `Response ${i}`, context)
        );
    }

    for (const count of [2, 5, 10]) {
        benchmark(`Merge ${count} patterns`, () => {
            coachEngine.mergePatterns(patterns.slice(0, count));
        }, 1000);
    }
}

/**
 * Benchmark style analysis
 */
function benchmarkStyleAnalysis() {
    console.log('\n=== Style Analysis Benchmarks ===\n');

    const messages = [
        "Hey! How's it going? 😊",
        "What's up?",
        "How are you doing today?",
        "Hope you're having a great day!",
        "Thanks so much for your help!",
    ];

    benchmark('Analyze style', () => {
        coachEngine.analyzeStyle(messages);
    }, 1000);
}

/**
 * Main benchmark runner
 */
function main() {
    console.log('='.repeat(60));
    console.log('COACH Engine Node.js Bindings - Performance Benchmarks');
    console.log('='.repeat(60));

    benchmarkPatternLearning();
    benchmarkStyleApplication();
    benchmarkPatternMerging();
    benchmarkStyleAnalysis();

    console.log('\n' + '='.repeat(60));
    console.log('Benchmarks Complete');
    console.log('='.repeat(60));
}

// Run benchmarks
main();
