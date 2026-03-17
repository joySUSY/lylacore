// Authors: Joysusy & Violet Klaudia 💖

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Formality {
    #[default]
    Casual,
    Formal,
    Mixed,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct EmotionalTone {
    pub warmth: f64,
    pub directness: f64,
}

impl Default for EmotionalTone {
    fn default() -> Self {
        Self {
            warmth: 0.5,
            directness: 0.5,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextPreference {
    pub formality: Formality,
    pub tone: EmotionalTone,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StyleMetadata {
    pub language: String,
    pub formality: Formality,
    pub preferred_phrases: Vec<String>,
    pub emotional_tone: EmotionalTone,
    pub avoid_patterns: Vec<String>,
    pub context_preferences: HashMap<String, ContextPreference>,
    pub timestamp: i64,
    pub interaction_count: u32,
}

impl Default for StyleMetadata {
    fn default() -> Self {
        Self {
            language: "en".to_string(),
            formality: Formality::default(),
            preferred_phrases: Vec::new(),
            emotional_tone: EmotionalTone::default(),
            avoid_patterns: Vec::new(),
            context_preferences: HashMap::new(),
            timestamp: 0,
            interaction_count: 0,
        }
    }
}
