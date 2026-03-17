// Authors: Joysusy & Violet Klaudia 💖

use crate::domain::helpers::{
    analyze_emotional_tone, analyze_formality_score, extract_phrases, merge_phrases,
    weighted_average,
};
use crate::domain::style_metadata::{ContextPreference, Formality, StyleMetadata};

const LEARNING_RATE: f64 = 0.3;

#[derive(Debug, Clone, Default)]
pub struct Context {
    pub language: Option<String>,
    pub topic: Option<String>,
}

#[must_use]
pub fn learn_pattern(
    user_message: &str,
    _agent_response: &str,
    context: &Context,
    existing_style: Option<StyleMetadata>,
) -> StyleMetadata {
    let mut style = existing_style.unwrap_or_default();

    if let Some(lang) = &context.language {
        style.language.clone_from(lang);
    }

    let formality_score = analyze_formality_score(user_message);
    style.formality = update_formality(formality_score);

    let phrases = extract_phrases(user_message);
    style.preferred_phrases = merge_phrases(&style.preferred_phrases, &phrases);

    let (warmth, directness) = analyze_emotional_tone(user_message);
    style.emotional_tone.warmth =
        weighted_average(style.emotional_tone.warmth, warmth, LEARNING_RATE);
    style.emotional_tone.directness =
        weighted_average(style.emotional_tone.directness, directness, LEARNING_RATE);

    if let Some(topic) = &context.topic {
        style
            .context_preferences
            .entry(topic.clone())
            .or_insert_with(|| ContextPreference {
                formality: style.formality,
                tone: style.emotional_tone,
            });
    }

    style.timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0))
        .as_millis()
        .try_into()
        .unwrap_or(i64::MAX);
    style.interaction_count += 1;

    style
}

const FORMALITY_CASUAL_THRESHOLD: f64 = 0.3;
const FORMALITY_FORMAL_THRESHOLD: f64 = 0.7;

fn update_formality(score: f64) -> Formality {
    if score < FORMALITY_CASUAL_THRESHOLD {
        Formality::Casual
    } else if score > FORMALITY_FORMAL_THRESHOLD {
        Formality::Formal
    } else {
        Formality::Mixed
    }
}
