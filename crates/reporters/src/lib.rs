//! Report formatters (Terminal, JSON) for A11y Copilot findings.

pub use a11y_core::domain_version;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reporters_link_to_core() {
        assert_eq!(domain_version(), "0.1.0");
    }
}
