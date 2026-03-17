// Authors: Joysusy & Violet Klaudia 💖

#[cfg(test)]
mod unit_tests {
    use crate::{
        analyze_style, apply_style, learn_pattern, merge_patterns, CoachError, Context, Formality,
        StyleMetadata,
    };

    #[test]
    fn test_learn_pattern_basic() {
        let context = Context {
            language: Some("en".to_string()),
            topic: Some("coding".to_string()),
        };

        let style = learn_pattern(
            "Hey, can you help me fix this bug?",
            "Sure, I'll help you debug it.",
            &context,
            None,
        );

        assert_eq!(style.language, "en");
        assert_eq!(style.interaction_count, 1);
        assert!(!style.preferred_phrases.is_empty());
    }

    #[test]
    fn test_learn_pattern_formality() {
        let context = Context {
            language: Some("en".to_string()),
            topic: None,
        };

        let casual_style = learn_pattern("hey yeah gonna do that", "response", &context, None);
        assert_eq!(casual_style.formality, Formality::Casual);

        let formal_style = learn_pattern(
            "Please kindly would you thank you",
            "response",
            &context,
            None,
        );
        assert_eq!(formal_style.formality, Formality::Formal);
    }

    #[test]
    fn test_apply_style_casual() {
        let style = StyleMetadata {
            formality: Formality::Casual,
            ..Default::default()
        };

        let result = apply_style("Please help me kindly", &style);
        assert!(!result.contains("Please"));
        assert!(!result.contains("kindly"));
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
    fn test_merge_patterns_empty() {
        let result = merge_patterns(&[]);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CoachError::EmptyPatternsArray
        ));
    }

    #[test]
    fn test_merge_patterns_single() {
        let style = StyleMetadata::default();
        let result = merge_patterns(std::slice::from_ref(&style));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), style);
    }

    #[test]
    fn test_merge_patterns_multiple() {
        let mut style1 = StyleMetadata::default();
        style1.emotional_tone.warmth = 0.3;
        style1.interaction_count = 5;

        let mut style2 = StyleMetadata::default();
        style2.emotional_tone.warmth = 0.7;
        style2.interaction_count = 3;

        let result = merge_patterns(&[style1, style2]).unwrap();
        assert_eq!(result.interaction_count, 8);
        assert!((result.emotional_tone.warmth - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_analyze_style() {
        let messages = vec!["hey yeah cool", "please kindly help", "thanks! wonderful!"];

        let profile = analyze_style(&messages);
        assert_eq!(profile.formality_distribution.casual, 1);
        assert_eq!(profile.formality_distribution.mixed, 2);
        assert!(profile.average_tone.warmth > 0.5);
    }

    #[test]
    fn test_analyze_style_empty() {
        let messages: Vec<&str> = vec![];
        let profile = analyze_style(&messages);
        assert_eq!(profile.formality_distribution.casual, 0);
    }
}
