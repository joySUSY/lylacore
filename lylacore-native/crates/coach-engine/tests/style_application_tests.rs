// Authors: Joysusy & Violet Klaudia 💖

use coach_engine::{apply_style, Formality, StyleMetadata};

#[test]
fn test_apply_style_casual() {
    let style = StyleMetadata {
        formality: Formality::Casual,
        ..Default::default()
    };

    let result = apply_style("Please help me kindly with this", &style);
    assert!(!result.contains("Please"));
    assert!(!result.contains("kindly"));
    assert!(result.contains("help me"));
}

#[test]
fn test_apply_style_formal() {
    let style = StyleMetadata {
        formality: Formality::Formal,
        ..Default::default()
    };

    let result = apply_style("hey yeah gonna do it", &style);
    assert!(result.contains("Hello"));
    assert!(result.contains("Yes"));
    assert!(result.contains("going to"));
}

#[test]
fn test_apply_style_warmth() {
    let mut style = StyleMetadata::default();
    style.emotional_tone.warmth = 0.8;

    let result = apply_style("Hello there", &style);
    assert!(result.contains("😊"));
}

#[test]
fn test_apply_style_avoid_patterns() {
    let style = StyleMetadata {
        avoid_patterns: vec!["bad".to_string(), "terrible".to_string()],
        ..Default::default()
    };

    let result = apply_style("This is bad and terrible", &style);
    assert!(!result.contains("bad"));
    assert!(!result.contains("terrible"));
    assert!(result.contains("This is"));
}

#[test]
fn test_apply_style_empty_message() {
    let style = StyleMetadata::default();
    let result = apply_style("", &style);
    assert_eq!(result, "");
}

#[test]
fn test_apply_style_unicode() {
    let style = StyleMetadata {
        formality: Formality::Casual,
        ..Default::default()
    };

    let result = apply_style("Please help 你好 世界", &style);
    assert!(result.contains("你好"));
    assert!(result.contains("世界"));
}

#[test]
fn test_apply_style_preserves_content() {
    let style = StyleMetadata::default();
    let original = "The quick brown fox jumps over the lazy dog";
    let result = apply_style(original, &style);

    assert!(result.contains("quick"));
    assert!(result.contains("brown"));
    assert!(result.contains("fox"));
}

#[test]
fn test_apply_style_multiple_patterns() {
    let mut style = StyleMetadata {
        formality: Formality::Casual,
        ..Default::default()
    };
    style.emotional_tone.warmth = 0.8;
    style.avoid_patterns = vec!["bad".to_string()];

    let result = apply_style("Please help, this is bad", &style);
    assert!(!result.contains("Please"));
    assert!(!result.contains("bad"));
    assert!(result.contains("😊"));
}
