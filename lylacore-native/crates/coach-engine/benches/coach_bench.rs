// Authors: Joysusy & Violet Klaudia 💖

use coach_engine::{analyze_style, apply_style, learn_pattern, merge_patterns, Context};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

fn bench_pattern_learning(c: &mut Criterion) {
    let mut group = c.benchmark_group("pattern_learning");

    let user_message = "Hey! How's it going? 😊";
    let agent_response = "I'm doing great, thanks for asking!";
    let context = Context {
        language: Some("en".to_string()),
        topic: Some("greeting".to_string()),
    };

    group.bench_function("learn_pattern_new", |b| {
        b.iter(|| {
            learn_pattern(
                black_box(user_message),
                black_box(agent_response),
                black_box(&context),
                black_box(None),
            )
        });
    });

    let existing_style = learn_pattern(user_message, agent_response, &context, None);

    group.bench_function("learn_pattern_update", |b| {
        b.iter(|| {
            learn_pattern(
                black_box(user_message),
                black_box(agent_response),
                black_box(&context),
                black_box(Some(existing_style.clone())),
            )
        });
    });

    group.finish();
}

fn bench_style_application(c: &mut Criterion) {
    let mut group = c.benchmark_group("style_application");

    let user_message = "Hey! How's it going? 😊";
    let agent_response = "I'm doing great, thanks for asking!";
    let context = Context {
        language: Some("en".to_string()),
        topic: Some("greeting".to_string()),
    };

    let style = learn_pattern(user_message, agent_response, &context, None);
    let message = "This is a test message for style application";

    group.bench_function("apply_style", |b| {
        b.iter(|| apply_style(black_box(message), black_box(&style)));
    });

    group.finish();
}

fn bench_pattern_merging(c: &mut Criterion) {
    let mut group = c.benchmark_group("pattern_merging");

    let context = Context {
        language: Some("en".to_string()),
        topic: Some("general".to_string()),
    };

    let patterns: Vec<_> = (0..10)
        .map(|i| {
            learn_pattern(
                &format!("Message {}", i),
                &format!("Response {}", i),
                &context,
                None,
            )
        })
        .collect();

    for count in [2, 5, 10].iter() {
        group.bench_with_input(BenchmarkId::new("merge", count), count, |b, &count| {
            b.iter(|| merge_patterns(black_box(&patterns[..count])));
        });
    }

    group.finish();
}

fn bench_style_analysis(c: &mut Criterion) {
    let mut group = c.benchmark_group("style_analysis");

    let messages = [
        "Hey! How's it going? 😊",
        "What's up?",
        "How are you doing today?",
        "Hope you're having a great day!",
        "Thanks so much for your help!",
    ];

    group.bench_function("analyze_style", |b| {
        b.iter(|| {
            analyze_style(black_box(
                &messages.iter().map(|s| s.as_ref()).collect::<Vec<_>>(),
            ))
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_pattern_learning,
    bench_style_application,
    bench_pattern_merging,
    bench_style_analysis
);
criterion_main!(benches);
