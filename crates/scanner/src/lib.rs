//! Scanner orchestration and engine adapters for A11y Copilot.

pub use a11y_core::domain_version;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scanner_links_to_core() {
        assert_eq!(domain_version(), "0.1.0");
    }
}
