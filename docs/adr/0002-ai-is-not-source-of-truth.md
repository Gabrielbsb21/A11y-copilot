# ADR 0002: AI Is Not the Sole Source of Accessibility Truth

- **Status:** Accepted
- **Date:** 2026-11-11

## Context

Large Language Models (LLMs) are helpful for interpreting UI context, explaining user impact in plain language, and proposing code fixes (for example, inferring whether an icon-only heart button means "Add to favorites" or "Like"). However, LLMs are probabilistic and can hallucinate issues, misread context, or claim compliance where a deterministic rule fails.

Developers using an accessibility assistant need to know with certainty **what was verified deterministically** versus **what was inferred by AI**.

## Decision

1. **Deterministic accessibility problems must be detected by deterministic engines, not by an LLM.**
2. **AI is used for contextual reasoning, impact explanation, and fix suggestion—never as the sole source of deterministic accessibility truth.**
3. The domain and reports must explicitly separate:
   - **Deterministic Facts (`Evidence`):** verifiable observations produced by rules/engines (e.g., `axe-core` rule `button-name` detected an empty accessible name on `<button>`).
   - **Contextual Interpretation (`AiAnalysis`):** probabilistic explanations and suggested fixes that always carry a `confidence` score and a `requires_human_review` flag.
4. All AI capabilities must be evaluated against a curated reference dataset (*Golden Dataset*) measuring factual grounding, fix usefulness, and hallucination rate.

## Consequences

- **Positive:** Prevents silent hallucinations from being presented as compliance facts; builds developer trust and makes AI quality measurable.
- **Negative:** Requires richer domain modeling and an evaluation pipeline (`evals/`) before scaling AI features.
