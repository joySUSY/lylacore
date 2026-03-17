// Authors: Joysusy & Violet Klaudia 💖

use crate::domain::helpers::{analyze_emotional_tone, analyze_formality_score};
use crate::domain::style_metadata::{EmotionalTone, StyleMetadata};
use crate::error::CoachError;

#[derive(Debug, Clone)]
pub struct StyleProfile {
    pub dominant_language: String,
    pub formality_distribution: FormalityDistribution,
    pub common_phrases: Vec<String>,
    pub average_tone: EmotionalTone,
}

#[derive(Debug, Clone)]
pub struct FormalityDistribution {
    pub casual: usize,
    pub formal: usize,
    pub mixed: usize,
}

pub fn analyze_style(messages: &[&str]) -> StyleProfile {
    let mut profile = StyleProfile {
        dominant_language: "en".to_string(),
        formality_distribution: FormalityDistribution {
            casual: 0,
            formal: 0,
            mixed: 0,
        },
        common_phrases: Vec::new(),
        average_tone: EmotionalTone {
            warmth: 0.0,
            directness: 0.0,
        },
    };

    if messages.is_empty() {
        return profile;
    }

    for message in messages {
        let formality_score = analyze_formality_score(message);
        if formality_score < 0.3 {
            profile.formality_distribution.casual += 1;
        } else if formality_score > 0.7 {
            profile.formality_distribution.formal += 1;
        } else {
            profile.formality_distribution.mixed += 1;
        }

        let (warmth, directness) = analyze_emotional_tone(message);
        profile.average_tone.warmth += warmth;
        profile.average_tone.directness += directness;
    }

    profile.average_tone.warmth /= messages.len() as f64;
    profile.average_tone.directness /= messages.len() as f64;

    profile
}

pub fn merge_patterns(patterns: &[StyleMetadata]) -> Result<StyleMetadata, CoachError> {
    if patterns.is_empty() {
        return Err(CoachError::EmptyPatternsArray);
    }

    if patterns.len() == 1 {
        return Ok(patterns[0].clone());
    }

    let mut sorted = patterns.to_vec();
    sorted.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    let mut merged = sorted[0].clone();

    let all_phrases: Vec<String> = patterns
        .iter()
        .flat_map(|p| p.preferred_phrases.clone())
        .collect();
    let unique_phrases: Vec<String> = all_phrases
        .into_iter()
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .take(20)
        .collect();
    merged.preferred_phrases = unique_phrases;

    let avg_warmth: f64 = patterns
        .iter()
        .map(|p| p.emotional_tone.warmth)
        .sum::<f64>()
        / patterns.len() as f64;
    let avg_directness: f64 = patterns
        .iter()
        .map(|p| p.emotional_tone.directness)
        .sum::<f64>()
        / patterns.len() as f64;
    merged.emotional_tone = EmotionalTone {
        warmth: avg_warmth,
        directness: avg_directness,
    };

    merged.interaction_count = patterns.iter().map(|p| p.interaction_count).sum();

    Ok(merged)
}
