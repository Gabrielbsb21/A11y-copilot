//! Contextual AI analysis and structured output validation for A11y Copilot.
//!
//! Per ADR 0002, AI is used for contextual explanation and fix suggestions,
//! never as the sole source of deterministic accessibility truth.

pub use a11y_core::domain_version;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ai_links_to_core() {
        assert_eq!(domain_version(), "0.1.0");
    }
}
