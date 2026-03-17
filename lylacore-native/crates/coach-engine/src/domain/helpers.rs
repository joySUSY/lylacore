// Authors: Joysusy & Violet Klaudia 💖

use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

const FORMALITY_WEIGHT: f64 = 0.1;
const WARMTH_WEIGHT: f64 = 0.05;
const DIRECTNESS_WEIGHT: f64 = 0.05;

#[must_use]
pub fn analyze_formality_score(text: &str) -> f64 {
    let formal_indicators = ["please", "kindly", "would you", "could you", "thank you"];
    let casual_indicators = ["hey", "yeah", "gonna", "wanna", "cool"];

    let mut score: f64 = 0.5;
    let lower = text.to_lowercase();

    for indicator in &formal_indicators {
        if lower.contains(indicator) {
            score += FORMALITY_WEIGHT;
        }
    }
    for indicator in &casual_indicators {
        if lower.contains(indicator) {
            score -= FORMALITY_WEIGHT;
        }
    }

    score.clamp(0.0, 1.0)
}

pub fn extract_phrases(text: &str) -> Vec<String> {
    let lower = text.to_lowercase();
    let words: Vec<&str> = lower.split_whitespace().collect();

    words
        .windows(2)
        .take(10)
        .map(|w| format!("{} {}", w[0], w[1]))
        .collect()
}

const MAX_MERGED_PHRASES: usize = 20;

#[must_use]
pub fn merge_phrases(existing: &[String], new_phrases: &[String]) -> Vec<String> {
    let mut counts: HashMap<String, usize> =
        HashMap::with_capacity(existing.len() + new_phrases.len());

    for phrase in existing.iter().chain(new_phrases.iter()) {
        *counts.entry(phrase.clone()).or_insert(0) += 1;
    }

    let mut sorted: Vec<_> = counts.into_iter().collect();
    sorted.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    sorted
        .into_iter()
        .take(MAX_MERGED_PHRASES)
        .map(|(phrase, _)| phrase)
        .collect()
}

#[must_use]
pub fn analyze_emotional_tone(text: &str) -> (f64, f64) {
    let warm_indicators = [
        "thanks",
        "appreciate",
        "love",
        "great",
        "wonderful",
        "!",
        "😊",
        "💜",
    ];
    let direct_indicators = ["need", "must", "should", "fix", "error", "problem"];

    let mut warmth: f64 = 0.5;
    let mut directness: f64 = 0.5;
    let lower = text.to_lowercase();

    for indicator in &warm_indicators {
        if lower.contains(indicator) {
            warmth += WARMTH_WEIGHT;
        }
    }
    for indicator in &direct_indicators {
        if lower.contains(indicator) {
            directness += DIRECTNESS_WEIGHT;
        }
    }

    (warmth.clamp(0.0, 1.0), directness.clamp(0.0, 1.0))
}

#[must_use]
pub fn weighted_average(old_value: f64, new_value: f64, weight: f64) -> f64 {
    old_value.mul_add(1.0 - weight, new_value * weight)
}

// Compile regex patterns once at startup (lazy static)
static RE_PLEASE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i)\bplease\b").unwrap());
static RE_KINDLY: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i)\bkindly\b").unwrap());
static RE_HEY: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i)\bhey\b").unwrap());
static RE_YEAH: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i)\byeah\b").unwrap());
static RE_GONNA: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i)\bgonna\b").unwrap());

#[must_use]
pub fn make_casual(text: &str) -> String {
    let result = RE_PLEASE.replace_all(text, "");
    let result = RE_KINDLY.replace_all(&result, "");

    result.trim().to_string()
}

#[must_use]
pub fn make_formal(text: &str) -> String {
    let result = RE_HEY.replace_all(text, "Hello");
    let result = RE_YEAH.replace_all(&result, "Yes");
    let result = RE_GONNA.replace_all(&result, "going to");

    result.to_string()
}

#[must_use]
pub fn add_warmth(text: &str) -> String {
    if !text.contains('!') && !text.contains("😊") {
        format!("{text} 😊")
    } else {
        text.to_string()
    }
}
