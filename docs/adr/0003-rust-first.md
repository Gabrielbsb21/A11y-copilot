# ADR 0003: Rust-First Architecture with Explicit Runtime Adapters

- **Status:** Accepted
- **Date:** 2026-11-11

## Context

A11y Copilot requires strong domain modeling, explicit error handling, fast CLI execution, predictable concurrency, and type-safe structured validation for AI outputs and evaluation pipelines. Rust excels at all of these.

At the same time, browser automation and DOM accessibility instrumentation (`Playwright` and `@axe-core/playwright`) are most mature in the Node.js ecosystem.

## Decision

1. **The core architecture of A11y Copilot is Rust-first** (`edition = "2024"`, `rust-version = "1.99"`).
2. Rust owns the CLI (`crates/cli`), domain model (`crates/core`), engine orchestration (`crates/scanner`), reporting (`crates/reporters`), AI provider abstractions (`crates/ai`), and evaluation harness.
3. Mature external ecosystems are used **only behind explicit process/adapter boundaries**. Specifically, browser automation runs in `browser/playwright-runner` on **Node.js 26 LTS** (using native TypeScript execution) invoked by Rust as a managed subprocess communicating via versioned JSON over `stdin`/`stdout`.

## Consequences

- **Positive:** Combines Rust's type safety, reliability, and performance in the core with Playwright's industry-standard browser automation.
- **Negative:** Introduces a multi-process boundary (Rust ↔ Node.js) that requires explicit timeout, process lifecycle, and contract-version error handling.
