# Contributing to A11y Copilot

## Core Principles

1. **Deterministic Fact vs. AI Interpretation ([ADR 0002](docs/adr/0002-ai-is-not-source-of-truth.md))**:
   - Deterministic accessibility issues must be detected by deterministic engines (`axe-core`, native rules, keyboard/tree analyzers).
   - AI is used for contextual explanation and suggested fixes, always accompanied by confidence and human-review metadata.
2. **Engine Isolation ([ADR 0001](docs/adr/0001-pluggable-accessibility-engines.md))**:
   - `crates/core` must never depend on `axe-core`, Playwright, or specific LLM provider schemas.
3. **Rust-First Architecture ([ADR 0003](docs/adr/0003-rust-first.md))**:
   - Use Rust 1.99+ (`edition = "2024"`) for core domain, CLI, orchestration, reporting, and evaluation.
   - Use Node.js 26 LTS (`browser/playwright-runner`) behind explicit versioned JSON contracts over `stdin`/`stdout`.

## Development Cycle

Every feature follows this engineering and learning workflow:

1. **Understand the problem**: Read or write the spec in `specs/<feature>/`.
2. **Define the expected result**: Document deterministic facts vs. contextual interpretations.
3. **Discuss the solution**: Verify crate boundaries and ADR compliance.
4. **Implement**: Write clean, type-safe code without `.unwrap()` in production paths.
5. **Test a failure**: Explicitly verify how the system behaves when inputs, subprocesses, or contracts fail.
6. **Document learnings**: Summarize architectural takeaways.

## Quality Checks (Definition of Done)

Before opening a pull request or completing a slice, ensure all checks pass:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```
