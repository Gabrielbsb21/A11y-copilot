# Requirements: URL Accessibility Scan (`url-scan`)

## Goal

Allow a developer to scan a web page URL (`http://`, `https://`, or local `file://` fixture) and receive a normalized accessibility report containing deterministic findings, affected elements, user impact, and verifiable engine evidence.

## Functional Requirements

1. **Target Validation (`FR-1`)**:
   - The system must validate the target URL before starting a browser session.
   - Supported schemes are `http://`, `https://`, and `file://`.
   - Empty or malformed targets must be rejected deterministically with a clear domain error.

2. **Deterministic Detection (`FR-2`)**:
   - Accessibility violations must be detected by a deterministic engine (starting with `axe-core` via `browser/playwright-runner`), never by an LLM.
   - Every `Finding` in the domain must contain at least one deterministic `Evidence` record identifying the engine name, rule ID, and observation summary.

3. **Engine Isolation (`FR-3`)**:
   - External engine schemas (`axe-core` JSON output) must be translated inside `crates/scanner` into `a11y_core::Finding`.
   - `crates/core` and `crates/reporters` must not depend on `axe-core` types.

4. **Fact vs. Interpretation Boundary (`FR-4`)**:
   - Deterministic facts (`Evidence`) and contextual AI interpretations (`AiInterpretation`) must be stored in separate fields of a `Finding`.
   - Any contextual interpretation must carry a bounded `Confidence` (`0.0..=1.0`) and an explicit `requires_human_review` flag.
