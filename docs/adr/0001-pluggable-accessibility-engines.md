# ADR 0001: Pluggable Accessibility Engines

- **Status:** Accepted
- **Date:** 2026-11-11

## Context

Automated web accessibility analysis relies on engines such as `axe-core`, native DOM/HTML rules, keyboard navigation testers, and accessibility tree inspectors. No single engine can detect every accessibility issue, and external tools evolve independently with their own JSON schemas and data models.

If the core domain of A11y Copilot depends directly on `axe-core` data structures, adding new engines (such as native Rust rules or behavioral keyboard checks) or replacing an engine would require invasive changes across the CLI, AI analyzer, evaluation suite, and reporters.

## Decision

1. **Accessibility engines must be replaceable and isolated behind explicit adapters.**
2. `crates/core` defines the canonical domain model (`Finding`, `Evidence`, `Severity`, `Category`, `ElementContext`, `ScanTarget`, `ScanReport`) and must have **zero dependencies** on `axe-core`, Playwright, or any specific engine format.
3. `crates/scanner` defines the `AccessibilityEngine` trait that returns domain `Finding` values.
4. External engines (starting with `axe-core` running inside `browser/playwright-runner`) communicate with the Rust adapter via a **versioned JSON contract over `stdin`/`stdout`** (with diagnostics restricted to `stderr`), and their raw outputs are normalized inside the adapter boundary.

## Consequences

- **Positive:** Domain logic, reporters, evaluations, and AI prompts remain decoupled from third-party schemas. Multiple engines can aggregate deterministic evidence into the same `Finding`.
- **Negative:** Requires writing and maintaining translation/normalization code in each engine adapter.
