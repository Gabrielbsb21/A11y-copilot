# A11y Copilot

**A11y Copilot** é um assistente open source de engenharia de acessibilidade *Rust-first* para desenvolvedores. Ele combina verificação determinística (via motores plugáveis como `axe-core`), coleta de contexto da interface e raciocínio assistido por IA avaliado continuamente (*Evaluation-Driven Development*).

## Princípio Central

O A11y Copilot separa explicitamente **fato determinístico** de **interpretação contextual**:

- **Fato determinístico (`Evidence`)**: o que um motor de análise comprovou na estrutura da página (ex.: `<button>` sem nome acessível).
- **Interpretação da IA (`AiAnalysis`)**: inferência baseada no contexto visual/textual ao redor do elemento (ex.: sugerir `aria-label="Adicionar aos favoritos"`), acompanhada de nível de confiança e indicação de revisão humana.

## Arquitetura do Workspace

```text
a11y-copilot/
├── crates/
│   ├── core/        # Domínio puro (Finding, Evidence, Severity, Category)
│   ├── scanner/     # Trait AccessibilityEngine e adapters (ex.: AxeEngineAdapter)
│   ├── ai/          # Trait AiProvider, prompts e validação de saídas estruturadas
│   ├── reporters/   # Formatadores de saída (Terminal e JSON)
│   └── cli/         # Binário `a11y`
├── browser/
│   └── playwright-runner/ # Subprocesso Node.js 26 LTS + Playwright + axe-core
├── fixtures/        # Páginas HTML de teste e casos ambíguos
├── evals/           # Golden dataset e avaliação da IA
├── specs/           # Especificações e critérios de aceite por funcionalidade
└── docs/
    └── adr/         # Architecture Decision Records
```

## Requisitos de Ambiente

- **Rust**: `1.99+` (`edition = "2024"`)
- **Node.js**: `26 LTS` (configurado em `.nvmrc`)

## Verificação Local

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Documentação e Decisões Arquiteturais

- [Resumo do Projeto](docs/a11y-copilot-project-summary.md)
- [Guia de Implementação](docs/a11y-copilot-implementation-guide.md)
- [ADR 0001 — Pluggable Accessibility Engines](docs/adr/0001-pluggable-accessibility-engines.md)
- [ADR 0002 — AI Is Not the Sole Source of Accessibility Truth](docs/adr/0002-ai-is-not-source-of-truth.md)
- [ADR 0003 — Rust-First Architecture](docs/adr/0003-rust-first.md)
