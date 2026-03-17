// Authors: Joysusy & Violet Klaudia 💖

use coach_engine::{
    analyze_style, apply_style, learn_pattern, merge_patterns, Context, Formality, StyleMetadata,
};

#[test]
fn test_full_learning_workflow() {
    let context = Context {
        language: Some("en".to_string()),
        topic: Some("coding".to_string()),
    };

    let style1 = learn_pattern(
        "Hey, can you help me with this bug?",
        "Sure, I'll help you debug it.",
        &context,
        None,
    );

    let style2 = learn_pattern(
        "Thanks! That's wonderful!",
        "You're welcome!",
        &context,
        Some(style1),
    );

    assert_eq!(style2.interaction_count, 2);
    assert!(style2.emotional_tone.warmth > 0.5);
}

#[test]
fn test_learn_and_apply_cycle() {
    let context = Context {
        language: Some("en".to_string()),
        topic: None,
    };

    let mut style = learn_pattern("hey yeah cool", "response", &context, None);
    style.formality = Formality::Casual;

    let result = apply_style("Please help me kindly", &style);
    assert!(!result.contains("Please"));
}

#[test]
fn test_merge_multiple_patterns() {
    let mut style1 = StyleMetadata::default();
    style1.emotional_tone.warmth = 0.3;
    style1.interaction_count = 5;
    style1.preferred_phrases = vec!["hello world".to_string()];

    let mut style2 = StyleMetadata::default();
    style2.emotional_tone.warmth = 0.7;
    style2.interaction_count = 3;
    style2.preferred_phrases = vec!["good morning".to_string()];

    let mut style3 = StyleMetadata::default();
    style3.emotional_tone.warmth = 0.5;
    style3.interaction_count = 2;

    let merged = merge_patterns(&[style1, style2, style3]).unwrap();
    assert_eq!(merged.interaction_count, 10);
    assert!((merged.emotional_tone.warmth - 0.5).abs() < 0.1);
}

#[test]
fn test_analyze_message_batch() {
    let messages = vec![
        "hey cool",
        "please help",
        "thanks!",
        "yeah gonna do it",
        "wonderful work",
    ];

    let profile = analyze_style(&messages);
    assert!(profile.formality_distribution.casual + profile.formality_distribution.mixed > 0);
    assert!(profile.average_tone.warmth > 0.5);
}

#[test]
fn test_incremental_style_evolution() {
    let context = Context {
        language: Some("en".to_string()),
        topic: None,
    };

    let mut style = StyleMetadata::default();

    for _ in 0..5 {
        style = learn_pattern("thanks! wonderful!", "response", &context, Some(style));
    }

    assert_eq!(style.interaction_count, 5);
    assert!(style.emotional_tone.warmth > 0.6);
}

#[test]
fn test_context_switching() {
    let coding_context = Context {
        language: Some("en".to_string()),
        topic: Some("coding".to_string()),
    };

    let design_context = Context {
        language: Some("en".to_string()),
        topic: Some("design".to_string()),
    };

    let mut style = learn_pattern("fix this bug", "response", &coding_context, None);
    style = learn_pattern(
        "make it beautiful",
        "response",
        &design_context,
        Some(style),
    );

    assert!(style.context_preferences.contains_key("coding"));
    assert!(style.context_preferences.contains_key("design"));
}

#[test]
fn test_multilingual_learning() {
    let en_context = Context {
        language: Some("en".to_string()),
        topic: None,
    };

    let zh_context = Context {
        language: Some("zh".to_string()),
        topic: None,
    };

    let mut style = learn_pattern("hello", "response", &en_context, None);
    assert_eq!(style.language, "en");

    style = learn_pattern("你好", "response", &zh_context, Some(style));
    assert_eq!(style.language, "zh");
}
