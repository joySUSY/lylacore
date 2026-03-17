// Authors: Joysusy & Violet Klaudia 💖

use crate::domain::helpers::{add_warmth, make_casual, make_formal};
use crate::domain::style_metadata::{Formality, StyleMetadata};
use regex::Regex;

const WARMTH_THRESHOLD: f64 = 0.7;

#[must_use]
pub fn apply_style(message: &str, style_metadata: &StyleMetadata) -> String {
    let mut styled = message.to_string();

    match style_metadata.formality {
        Formality::Casual => {
            styled = make_casual(&styled);
        }
        Formality::Formal => {
            styled = make_formal(&styled);
        }
        Formality::Mixed => {}
    }

    if style_metadata.emotional_tone.warmth > WARMTH_THRESHOLD {
        styled = add_warmth(&styled);
    }

    if !style_metadata.avoid_patterns.is_empty() {
        for pattern in &style_metadata.avoid_patterns {
            if let Ok(re) = Regex::new(&format!("(?i){}", regex::escape(pattern))) {
                styled = re.replace_all(&styled, "").into_owned();
            }
        }
    }

    styled.trim().to_string()
}
