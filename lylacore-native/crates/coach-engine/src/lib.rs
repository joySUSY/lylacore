// Authors: Joysusy & Violet Klaudia 💖
//! # COACH Protocol Engine
//!
//! **C**ontextual **O**bservation and **A**daptive **C**ommunication **H**armonization
//!
//! This engine learns from user-agent interactions and adapts communication patterns.
//! It is agent-agnostic - any agent can use it to learn their user's style.
//!
//! ## Core Functions
//!
//! - [`learn_pattern`]: Learn communication patterns from interactions
//! - [`apply_style`]: Apply learned style to new messages
//! - [`analyze_style`]: Analyze style metadata for insights
//!
//! ## Example
//!
//! ```rust
//! use coach_engine::{learn_pattern, apply_style, Context};
//!
//! let context = Context {
//!     language: Some("en".to_string()),
//!     topic: Some("coding".to_string()),
//! };
//!
//! // Learn from interaction
//! let style = learn_pattern(
//!     "How do I fix this bug?",
//!     "Let's debug it step by step.",
//!     &context,
//!     None,
//! );
//!
//! // Apply learned style
//! let adapted = apply_style("What should I do next?", &style);
//! ```

pub mod domain;
pub mod error;

#[cfg(test)]
mod tests;

pub use domain::analysis::{analyze_style, merge_patterns, FormalityDistribution, StyleProfile};
pub use domain::pattern_learning::{learn_pattern, Context};
pub use domain::style_application::apply_style;
pub use domain::style_metadata::{ContextPreference, EmotionalTone, Formality, StyleMetadata};
pub use error::CoachError;
