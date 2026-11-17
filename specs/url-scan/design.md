# Design: Core Domain & URL Scan Pipeline

## Domain Invariants (`crates/core`)

To follow the Rust principle **"Parse, don't validate"**, invalid states are rejected at construction time:

1. `ScanTarget`:
   - Inner `url: String` field is private.
   - Constructed via `ScanTarget::new(...) -> Result<ScanTarget, DomainError>`.
   - Guarantees non-empty URL with `http://`, `https://`, or `file://` scheme.
2. `Confidence`:
   - Inner `f32` field is private.
   - Constructed via `Confidence::new(score) -> Result<Confidence, DomainError>`.
   - Guarantees finite floating-point number within `0.0..=1.0`.
3. `Finding`:
   - Separates `evidence: Vec<Evidence>` (deterministic facts) from `interpretation: Option<AiInterpretation>` (contextual AI analysis).
   - `Finding::new(...)` enforces `!evidence.is_empty()`.
