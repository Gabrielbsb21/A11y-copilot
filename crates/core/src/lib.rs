//! Core domain model for A11y Copilot.
//!
//! This crate intentionally has zero dependencies on external accessibility engines
//! (such as `axe-core`), browser automation tools (`Playwright`), or AI providers.
//!
//! # Core Invariants
//! - **Engine Agnosticism ([ADR 0001])**: All external results normalize into [`Finding`].
//! - **Fact vs. Interpretation ([ADR 0002])**: Every [`Finding`] requires at least one
//!   deterministic [`Evidence`] entry; probabilistic AI output lives strictly inside
//!   [`AiInterpretation`] with a validated [`Confidence`] (`0.0..=1.0`).

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Canonical version identifier for the A11y Copilot domain crate.
#[must_use]
pub const fn domain_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Errors produced when validating domain invariants.
#[derive(Debug, Clone, PartialEq, Error)]
pub enum DomainError {
    /// Target URL was empty or contained only whitespace.
    #[error("scan target URL cannot be empty")]
    EmptyTargetUrl,

    /// Target URL did not start with `http://`, `https://`, or `file://` or lacked a path/host.
    #[error("unsupported or malformed target URL `{0}` (expected http://, https://, or file://)")]
    UnsupportedUrlScheme(String),

    /// Confidence score was outside the valid `0.0..=1.0` range or `NaN`.
    #[error("confidence score `{0}` must be a finite number between 0.0 and 1.0")]
    InvalidConfidenceScore(f32),

    /// A finding was constructed without any deterministic evidence (violates ADR 0002).
    #[error("finding `{0}` must contain at least one deterministic evidence record")]
    MissingDeterministicEvidence(String),
}

/// A validated target URL to be analyzed by accessibility engines.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScanTarget {
    url: String,
}

impl ScanTarget {
    /// Validates and constructs a [`ScanTarget`].
    ///
    /// Supported schemes are `http://`, `https://`, and `file://`.
    ///
    /// # Errors
    /// Returns [`DomainError::EmptyTargetUrl`] if the input is blank, or
    /// [`DomainError::UnsupportedUrlScheme`] if the scheme is missing/unsupported.
    pub fn new(raw_url: impl Into<String>) -> Result<Self, DomainError> {
        let trimmed = raw_url.into().trim().to_owned();
        if trimmed.is_empty() {
            return Err(DomainError::EmptyTargetUrl);
        }

        let remainder = trimmed
            .strip_prefix("https://")
            .or_else(|| trimmed.strip_prefix("http://"))
            .or_else(|| trimmed.strip_prefix("file://"));

        match remainder {
            Some(rest) if !rest.trim().is_empty() => Ok(Self { url: trimmed }),
            _ => Err(DomainError::UnsupportedUrlScheme(trimmed)),
        }
    }

    /// Returns the validated URL string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.url
    }
}

impl TryFrom<&str> for ScanTarget {
    type Error = DomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for ScanTarget {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// Severity level of an accessibility finding, ordered from lowest to highest impact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    /// Minor friction for assistive technology users.
    Minor,
    /// Moderate barrier that makes interaction slower or confusing.
    Moderate,
    /// Serious barrier that significantly degrades usability for affected users.
    Serious,
    /// Critical blocker that prevents affected users from completing a task.
    Critical,
}

/// Canonical accessibility problem category owned by A11y Copilot.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    /// Interactive element lacks an accessible name (`button-name`, `link-name`, etc.).
    MissingAccessibleName,
    /// Informative image or graphic lacks a text alternative (`image-alt`, `svg-img-alt`).
    MissingTextAlternative,
    /// Form input lacks an associated programmatic label (`label`).
    MissingFormLabel,
    /// Heading hierarchy skips levels or lacks structure (`heading-order`).
    HeadingOrder,
    /// Element acts interactively without proper role or keyboard semantics.
    InteractiveSemantics,
    /// Foreground/background text contrast is insufficient (`color-contrast`).
    ColorContrast,
    /// Any other rule category not yet promoted to a first-class variant.
    Other(String),
}

/// A deterministic observation recorded by an accessibility engine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Evidence {
    /// Name of the engine that produced this fact (e.g., `"axe-core"`, `"native"`).
    pub engine: String,
    /// Identifier of the deterministic rule (e.g., `"button-name"`).
    pub rule_id: String,
    /// Human-readable description of what the engine verified.
    pub summary: String,
}

/// Structural and contextual metadata about the DOM element affected by a finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ElementContext {
    /// CSS selector path identifying the element on the page.
    pub selector: String,
    /// Raw HTML snippet of the target element.
    pub html_snippet: String,
    /// Computed accessible role, if available.
    pub role: Option<String>,
    /// Computed accessible name, if available.
    pub accessible_name: Option<String>,
    /// Nearby visible text collected from parent/sibling containers (enriched in M2).
    pub nearby_text: Option<String>,
}

/// A bounded confidence score in the closed interval `0.0..=1.0`.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(try_from = "f32", into = "f32")]
pub struct Confidence(f32);

impl Confidence {
    /// Constructs a validated [`Confidence`] score within `0.0..=1.0`.
    ///
    /// # Errors
    /// Returns [`DomainError::InvalidConfidenceScore`] if `score` is `NaN` or outside `0.0..=1.0`.
    pub fn new(score: f32) -> Result<Self, DomainError> {
        if score.is_finite() && (0.0..=1.0).contains(&score) {
            Ok(Self(score))
        } else {
            Err(DomainError::InvalidConfidenceScore(score))
        }
    }

    /// Returns the underlying `f32` value (`0.0..=1.0`).
    #[must_use]
    pub const fn value(self) -> f32 {
        self.0
    }
}

impl TryFrom<f32> for Confidence {
    type Error = DomainError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<Confidence> for f32 {
    fn from(confidence: Confidence) -> Self {
        confidence.0
    }
}

/// Probabilistic contextual interpretation generated by an AI provider (M3).
///
/// Always kept separate from deterministic [`Evidence`] per ADR 0002.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AiInterpretation {
    /// Inferred intent of the UI element in its surrounding context (e.g., `"Adicionar aos favoritos"`).
    pub possible_intent: String,
    /// Contextual explanation of why the issue matters in this specific interface.
    pub explanation: String,
    /// Suggested code or attribute fix grounded in the element context.
    pub suggested_fix: String,
    /// Model confidence in this interpretation (`0.0..=1.0`).
    pub confidence: Confidence,
    /// Whether a human developer must verify the inferred intent before applying the fix.
    pub requires_human_review: bool,
}

/// An accessibility issue normalized into the A11y Copilot domain.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Finding {
    /// Stable identifier for this finding within a scan report.
    pub id: String,
    /// High-level accessibility category.
    pub category: Category,
    /// Impact severity for end users.
    pub severity: Severity,
    /// Concise statement of the problem (e.g., `"O botão não possui nome acessível"`).
    pub title: String,
    /// Description of who is affected and how (e.g., `"Pessoas usuárias de leitor de tela..."`).
    pub user_impact: String,
    /// Target DOM element and its captured context.
    pub element: ElementContext,
    /// Deterministic facts verified by one or more accessibility engines.
    pub evidence: Vec<Evidence>,
    /// Optional contextual interpretation produced by AI (populated in M3).
    pub interpretation: Option<AiInterpretation>,
}

impl Finding {
    /// Constructs a [`Finding`], enforcing that at least one deterministic [`Evidence`] exists.
    ///
    /// # Errors
    /// Returns [`DomainError::MissingDeterministicEvidence`] if `evidence` is empty.
    pub fn new(
        id: impl Into<String>,
        category: Category,
        severity: Severity,
        title: impl Into<String>,
        user_impact: impl Into<String>,
        element: ElementContext,
        evidence: Vec<Evidence>,
    ) -> Result<Self, DomainError> {
        let id = id.into();
        if evidence.is_empty() {
            return Err(DomainError::MissingDeterministicEvidence(id));
        }

        Ok(Self {
            id,
            category,
            severity,
            title: title.into(),
            user_impact: user_impact.into(),
            element,
            evidence,
            interpretation: None,
        })
    }

    /// Attaches a contextual [`AiInterpretation`] to this finding without altering its deterministic evidence.
    #[must_use]
    pub fn with_interpretation(mut self, interpretation: AiInterpretation) -> Self {
        self.interpretation = Some(interpretation);
        self
    }
}

/// Aggregated counts of findings grouped by [`Severity`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SeveritySummary {
    /// Total number of findings across all severities.
    pub total: usize,
    /// Count of [`Severity::Critical`] findings.
    pub critical: usize,
    /// Count of [`Severity::Serious`] findings.
    pub serious: usize,
    /// Count of [`Severity::Moderate`] findings.
    pub moderate: usize,
    /// Count of [`Severity::Minor`] findings.
    pub minor: usize,
}

/// Complete result of scanning a [`ScanTarget`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScanReport {
    /// Target URL that was analyzed.
    pub target: ScanTarget,
    /// Document `<title>` captured from the page, if present.
    pub page_title: Option<String>,
    /// Normalized findings discovered during the scan.
    pub findings: Vec<Finding>,
}

impl ScanReport {
    /// Creates a new [`ScanReport`].
    #[must_use]
    pub const fn new(
        target: ScanTarget,
        page_title: Option<String>,
        findings: Vec<Finding>,
    ) -> Self {
        Self {
            target,
            page_title,
            findings,
        }
    }

    /// Computes the [`SeveritySummary`] for all findings in this report.
    #[must_use]
    pub fn summary(&self) -> SeveritySummary {
        let mut summary = SeveritySummary {
            total: self.findings.len(),
            ..SeveritySummary::default()
        };

        for finding in &self.findings {
            match finding.severity {
                Severity::Critical => summary.critical += 1,
                Severity::Serious => summary.serious += 1,
                Severity::Moderate => summary.moderate += 1,
                Severity::Minor => summary.minor += 1,
            }
        }

        summary
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_evidence() -> Evidence {
        Evidence {
            engine: "axe-core".to_owned(),
            rule_id: "button-name".to_owned(),
            summary: "Element does not have inner text that is visible to screen readers"
                .to_owned(),
        }
    }

    fn sample_element() -> ElementContext {
        ElementContext {
            selector: "#fav-btn".to_owned(),
            html_snippet: "<button id=\"fav-btn\"><svg aria-hidden=\"true\"></svg></button>"
                .to_owned(),
            role: Some("button".to_owned()),
            accessible_name: None,
            nearby_text: Some("Fone Bluetooth Cancelamento de Ruído R$ 599,00".to_owned()),
        }
    }

    #[test]
    fn domain_version_is_not_empty() {
        assert_ne!(domain_version(), "");
    }

    #[test]
    fn scan_target_accepts_http_https_and_file_urls() {
        for valid in [
            "http://localhost:3000",
            "https://example.com/app",
            "file:///Users/dev/fixtures/ambiguous-buttons.html",
        ] {
            let target = ScanTarget::try_from(valid);
            assert!(target.is_ok(), "expected `{valid}` to be valid");
            assert_eq!(target.as_ref().map(ScanTarget::as_str), Ok(valid));
        }
    }

    #[test]
    fn scan_target_rejects_empty_or_unsupported_schemes() {
        assert_eq!(ScanTarget::new("   "), Err(DomainError::EmptyTargetUrl));
        assert_eq!(
            ScanTarget::new("ftp://example.com"),
            Err(DomainError::UnsupportedUrlScheme(
                "ftp://example.com".to_owned()
            ))
        );
        assert_eq!(
            ScanTarget::new("https://"),
            Err(DomainError::UnsupportedUrlScheme("https://".to_owned()))
        );
    }

    #[test]
    fn confidence_enforces_unit_interval() {
        assert!(Confidence::new(0.0).is_ok());
        assert!(Confidence::new(0.78).is_ok());
        assert!(Confidence::new(1.0).is_ok());

        assert_eq!(
            Confidence::new(-0.01),
            Err(DomainError::InvalidConfidenceScore(-0.01))
        );
        assert_eq!(
            Confidence::new(1.01),
            Err(DomainError::InvalidConfidenceScore(1.01))
        );
        assert!(matches!(
            Confidence::new(f32::NAN),
            Err(DomainError::InvalidConfidenceScore(_))
        ));
    }

    #[test]
    fn confidence_rejects_out_of_bounds_during_json_deserialization() {
        let valid_json = "0.85";
        let parsed: Result<Confidence, _> = serde_json::from_str(valid_json);
        assert!(parsed.is_ok());

        let invalid_json = "1.45";
        let rejected: Result<Confidence, _> = serde_json::from_str(invalid_json);
        assert!(rejected.is_err());
    }

    #[test]
    fn finding_requires_at_least_one_deterministic_evidence() {
        let result = Finding::new(
            "finding-1",
            Category::MissingAccessibleName,
            Severity::Serious,
            "O botão não possui nome acessível",
            "Pessoas usando leitor de tela não conseguem identificar a ação do botão.",
            sample_element(),
            vec![],
        );

        assert_eq!(
            result,
            Err(DomainError::MissingDeterministicEvidence(
                "finding-1".to_owned()
            ))
        );
    }

    #[test]
    fn report_summary_aggregates_severities_and_serializes_cleanly() -> Result<(), DomainError> {
        let target = ScanTarget::new("http://localhost:3000")?;
        let confidence = Confidence::new(0.78)?;

        let finding = Finding::new(
            "finding-1",
            Category::MissingAccessibleName,
            Severity::Serious,
            "O botão não possui nome acessível",
            "Uma pessoa usando leitor de tela pode não identificar a ação.",
            sample_element(),
            vec![sample_evidence()],
        )?
        .with_interpretation(AiInterpretation {
            possible_intent: "Adicionar aos favoritos".to_owned(),
            explanation: "O botão está dentro de um card de produto ao lado de 'Comprar agora'."
                .to_owned(),
            suggested_fix: "<button id=\"fav-btn\" aria-label=\"Adicionar aos favoritos\">"
                .to_owned(),
            confidence,
            requires_human_review: true,
        });

        let report = ScanReport::new(target, Some("Vitrine".to_owned()), vec![finding]);
        let summary = report.summary();

        assert_eq!(
            summary,
            SeveritySummary {
                total: 1,
                critical: 0,
                serious: 1,
                moderate: 0,
                minor: 0,
            }
        );

        let serialized = serde_json::to_string_pretty(&report);
        assert!(serialized.is_ok());

        Ok(())
    }
}
