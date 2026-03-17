// Authors: Joysusy & Violet Klaudia 💖

use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::collections::HashMap;

#[napi(object)]
pub struct StyleMetadataJs {
    pub language: String,
    pub formality: String,
    pub preferred_phrases: Vec<String>,
    pub emotional_tone: EmotionalToneJs,
    pub avoid_patterns: Vec<String>,
    pub context_preferences: String,
    pub timestamp: f64,
    pub interaction_count: f64,
}

#[napi(object)]
pub struct EmotionalToneJs {
    pub warmth: f64,
    pub directness: f64,
}

#[napi(object)]
pub struct InteractionContextJs {
    pub language: Option<String>,
    pub topic: Option<String>,
}

#[napi]
pub fn learn_pattern(
    user_message: String,
    agent_response: String,
    context: InteractionContextJs,
    existing_style: Option<StyleMetadataJs>,
) -> Result<StyleMetadataJs> {
    let ctx = coach_engine::Context {
        language: context.language,
        topic: context.topic,
    };

    let existing = existing_style.map(js_to_style_metadata_owned);

    let result = coach_engine::learn_pattern(&user_message, &agent_response, &ctx, existing);

    Ok(style_metadata_to_js(&result))
}

#[napi]
pub fn apply_style(message: String, style_metadata: StyleMetadataJs) -> Result<String> {
    let style = js_to_style_metadata_borrowed(&style_metadata);

    Ok(coach_engine::apply_style(&message, &style))
}

// Consolidated conversion function to eliminate duplication
fn js_to_style_metadata_impl(
    language: String,
    formality_str: &str,
    preferred_phrases: Vec<String>,
    emotional_tone: &EmotionalToneJs,
    avoid_patterns: Vec<String>,
    context_preferences_json: &str,
    timestamp: f64,
    interaction_count: f64,
) -> coach_engine::StyleMetadata {
    use coach_engine::{EmotionalTone, Formality, StyleMetadata};

    let formality = match formality_str {
        "casual" => Formality::Casual,
        "formal" => Formality::Formal,
        _ => Formality::Mixed,
    };

    let context_preferences: HashMap<String, coach_engine::ContextPreference> =
        serde_json::from_str(context_preferences_json).unwrap_or_default();

    StyleMetadata {
        language,
        formality,
        preferred_phrases,
        emotional_tone: EmotionalTone {
            warmth: emotional_tone.warmth,
            directness: emotional_tone.directness,
        },
        avoid_patterns,
        context_preferences,
        timestamp: timestamp as i64,
        interaction_count: interaction_count as u32,
    }
}

fn js_to_style_metadata_owned(js: StyleMetadataJs) -> coach_engine::StyleMetadata {
    js_to_style_metadata_impl(
        js.language,
        &js.formality,
        js.preferred_phrases,
        &js.emotional_tone,
        js.avoid_patterns,
        &js.context_preferences,
        js.timestamp,
        js.interaction_count,
    )
}

fn js_to_style_metadata_borrowed(js: &StyleMetadataJs) -> coach_engine::StyleMetadata {
    js_to_style_metadata_impl(
        js.language.clone(),
        &js.formality,
        js.preferred_phrases.clone(),
        &js.emotional_tone,
        js.avoid_patterns.clone(),
        &js.context_preferences,
        js.timestamp,
        js.interaction_count,
    )
}

fn style_metadata_to_js(style: &coach_engine::StyleMetadata) -> StyleMetadataJs {
    let formality = match style.formality {
        coach_engine::Formality::Casual => "casual".to_string(),
        coach_engine::Formality::Formal => "formal".to_string(),
        coach_engine::Formality::Mixed => "mixed".to_string(),
    };

    StyleMetadataJs {
        language: style.language.clone(),
        formality,
        preferred_phrases: style.preferred_phrases.clone(),
        emotional_tone: EmotionalToneJs {
            warmth: style.emotional_tone.warmth,
            directness: style.emotional_tone.directness,
        },
        avoid_patterns: style.avoid_patterns.clone(),
        context_preferences: serde_json::to_string(&style.context_preferences)
            .unwrap_or_else(|_| "{}".to_string()),
        timestamp: style.timestamp as f64,
        interaction_count: style.interaction_count as f64,
    }
}
