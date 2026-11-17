# A11y Copilot - Guia para Iniciar a Implementação

Este documento descreve uma forma prática de iniciar o A11y Copilot com uma base arquitetural sólida e um fluxo de desenvolvimento assistido por IA.

---

# 1. Objetivo da primeira implementação

A primeira versão não precisa resolver acessibilidade por completo.

Ela deve provar este fluxo:

```text
URL
 |
Playwright
 |
axe-core
 |
Axe Adapter
 |
A11y Copilot Finding
 |
Context Collector
 |
AI Analyzer
 |
Terminal / JSON Report
```

Quando esse pipeline estiver funcionando de ponta a ponta, o projeto já terá uma fundação real.

---

# 2. Criar o workspace Rust

Estrutura inicial:

```text
a11y-copilot/
|
|-- Cargo.toml
|
|-- crates/
|   |-- cli/
|   |-- core/
|   |-- scanner/
|   |-- ai/
|   `-- reporters/
|
|-- browser/
|   `-- playwright-runner/
|
|-- fixtures/
|-- specs/
|-- evals/
`-- docs/
    `-- adr/
```

Workspace:

```toml
[workspace]
resolver = "2"

members = [
    "crates/cli",
    "crates/core",
    "crates/scanner",
    "crates/ai",
    "crates/reporters"
]
```

---

# 3. Responsabilidade dos primeiros crates

## crates/core

Não deve depender de:

- axe-core;
- Playwright;
- OpenAI;
- Gemini;
- CLI;
- JSON específico de providers.

Contém apenas domínio.

Exemplos:

```text
Finding
Severity
Category
Evidence
ElementContext
Confidence
ScanTarget
ScanResult
```

---

## crates/scanner

Responsável por:

- definir abstrações de scanner;
- executar adapters;
- normalizar resultados;
- orquestrar engines.

Trait inicial:

```rust
#[async_trait]
pub trait AccessibilityEngine {
    async fn analyze(
        &self,
        target: &ScanTarget,
    ) -> Result<Vec<Finding>, EngineError>;
}
```

---

## crates/ai

Responsável por:

- interfaces de providers;
- structured output;
- prompts;
- validação;
- confidence;
- futuramente retrieval e reranking.

Trait:

```rust
#[async_trait]
pub trait AiProvider {
    async fn analyze(
        &self,
        input: &AccessibilityAnalysisInput,
    ) -> Result<AiAnalysis, AiError>;
}
```

---

## crates/reporters

Primeiros reporters:

```text
TerminalReporter
JsonReporter
```

Futuramente:

```text
MarkdownReporter
SarifReporter
GitHubReporter
```

---

## crates/cli

Comando inicial:

```bash
a11y scan http://localhost:3000
```

Depois:

```bash
a11y scan URL --format json
a11y scan URL --engine axe
a11y scan URL --engine all
a11y eval
```

---

# 4. Browser runner

Começar separado do Rust.

```text
browser/playwright-runner/
|
|-- package.json
|-- tsconfig.json
`-- src/
    `-- scan.ts
```

Entrada:

```json
{
  "url": "http://localhost:3000"
}
```

Saída:

```json
{
  "url": "http://localhost:3000",
  "title": "Application",
  "violations": []
}
```

Rust executa esse runner inicialmente como subprocesso.

O contrato deve ser versionado.

Nunca espalhar estruturas de axe-core pelo projeto inteiro.

---

# 5. Primeiro ADR

Criar:

```text
docs/adr/0001-pluggable-accessibility-engines.md
```

Decisão:

> Accessibility engines must be replaceable.

Consequência:

O domínio não depende diretamente de axe-core.

---

# 6. Segundo ADR

```text
docs/adr/0002-ai-is-not-source-of-truth.md
```

Decisão:

> AI is used for contextual reasoning and assistance, not as the sole source of deterministic accessibility truth.

---

# 7. Terceiro ADR

```text
docs/adr/0003-rust-first.md
```

Decisão:

> The core is Rust-first. External ecosystems may be used behind explicit adapters when they provide mature capabilities.

---

# 8. Primeira milestone

## M0 - Repository foundation

Entregáveis:

- Cargo workspace;
- crates vazios;
- CI;
- lint;
- fmt;
- tests;
- README;
- ADRs;
- contribution guide.

Definition of Done:

```text
cargo fmt --check
cargo clippy
cargo test
```

passam no CI.

---

# 9. Segunda milestone

## M1 - Scan a URL

Entregáveis:

- CLI;
- Playwright runner;
- axe-core;
- adapter;
- modelo interno `Finding`;
- relatório terminal.

Comando:

```bash
a11y scan https://example.com
```

Saída mínima:

```text
A11y Copilot

Target:
https://example.com

Findings: 4

Critical: 1
Serious:  2
Moderate: 1
```

---

# 10. Terceira milestone

## M2 - Context Collector

Para cada finding, capturar:

```text
element HTML
selector
parent context
nearby text
role
accessible name
page title
URL
```

Evitar enviar página inteira à IA.

---

# 11. Quarta milestone

## M3 - AI Explanation

Adicionar um único provider.

O provider deve gerar structured output.

Exemplo:

```json
{
  "explanation": "...",
  "affected_users": ["screen reader users"],
  "suggested_fix": {
    "description": "...",
    "code": "..."
  },
  "confidence": 0.91,
  "requires_human_review": false,
  "reasoning_category": "contextual"
}
```

---

# 12. Quinta milestone

## M4 - Evals

Antes de RAG ou agentes, criar dataset.

Primeiros grupos:

```text
button-name
image-alt
form-label
headings
interactive-elements
```

Cada caso deve armazenar:

- input;
- contexto;
- expected facts;
- allowed interpretations;
- forbidden hallucinations;
- human-review expectation.

---

# 13. Fluxo de desenvolvimento com IA

O fluxo recomendado é:

# Spec-Driven Development + AI-Assisted TDD + Evaluation-Driven Development

```text
Idea
 |
Spec
 |
Acceptance Criteria
 |
Tests
 |
Implementation Plan
 |
AI-assisted Implementation
 |
Automated Tests
 |
Evals
 |
Human Review
 |
ADR / Docs Update
```

---

# 14. Feature spec

Para cada feature:

```text
specs/<feature>/
|-- requirements.md
|-- acceptance.md
|-- design.md
`-- tasks.md
```

Exemplo:

```text
specs/button-name-rule/
```

---

# 15. Requirements

Exemplo:

```md
# Requirement

The system must detect an interactive button that has no accessible name.

The detection must not rely on an LLM.
```

---

# 16. Acceptance Criteria

Exemplo:

```md
Given:

<button>
  <svg />
</button>

When:

the page is scanned

Then:

the system creates a finding for missing accessible name.

And:

the finding contains deterministic evidence.

And:

AI may suggest a contextual label.

But:

the contextual label must not be represented as a deterministic fact.
```

---

# 17. Tests before implementation

Criar:

- unit tests;
- integration tests;
- fixture pages;
- snapshot tests.

A IA pode ajudar a gerar casos, mas os acceptance criteria devem orientar o trabalho.

---

# 18. Como usar um agente de código

Dar contexto controlado.

Exemplo de prompt de tarefa:

```text
Implement the feature described in:

specs/button-name-rule/requirements.md
specs/button-name-rule/acceptance.md
specs/button-name-rule/design.md

Respect:
docs/adr/0001-pluggable-accessibility-engines.md
docs/adr/0002-ai-is-not-source-of-truth.md

Before implementation:
1. inspect the current architecture;
2. propose a minimal implementation plan;
3. identify affected crates;
4. add or update tests;
5. implement the feature;
6. run fmt, clippy and tests;
7. summarize architectural impact.

Do not modify unrelated modules.
Do not expose axe-core types outside its adapter.
```

Isso é muito mais robusto do que:

```text
"implemente a regra"
```

---

# 19. Fluxo de branch

Exemplo:

```text
main
 |
feature/button-name-rule
 |
spec
 |
tests
 |
implementation
 |
eval
 |
review
 |
merge
```

Commits:

```text
feat(scanner): add accessible-name finding
test(scanner): add button-name fixtures
docs(adr): describe engine boundary
```

---

# 20. Definition of Done

Uma feature só está pronta quando:

- acceptance criteria foram atendidos;
- testes passam;
- clippy passa;
- fmt passa;
- integração funciona;
- evals relevantes não regrediram;
- documentação foi atualizada;
- nenhum provider externo vazou para o domínio;
- findings distinguem fato de interpretação.

---

# 21. AI Engineering workflow

Para features de IA:

```text
Dataset
 |
Baseline
 |
Prompt / Pipeline
 |
Structured Output
 |
Automated Checks
 |
LLM Judge (quando aplicável)
 |
Human Review
 |
Metrics
 |
Regression Gate
```

---

# 22. Golden Dataset

Criar exemplos de alta qualidade revisados manualmente.

Categorias:

```text
accessible examples
inaccessible examples
ambiguous examples
false-positive traps
framework-specific examples
```

Guardar expected outputs relevantes.

---

# 23. Métricas

Não medir apenas "parece bom".

Medir:

```text
factual correctness
grounding
fix correctness
hallucination rate
human-review precision
false-positive rate
confidence calibration
```

---

# 24. LLM-as-a-Judge

Usar somente onde métricas determinísticas não bastam.

Exemplos:

- clareza da explicação;
- utilidade da sugestão;
- coerência contextual.

Nunca usar LLM-as-a-Judge como única avaliação.

Combinar:

```text
deterministic checks
+
golden dataset
+
LLM judge
+
human review
```

---

# 25. RAG

Adicionar somente depois de existir:

- pipeline funcional;
- dataset;
- evals baseline.

Knowledge base futura:

```text
WCAG
WAI-ARIA APG
MDN
framework docs
rule documentation
```

Pipeline:

```text
Finding
 |
Query Builder
 |
Retriever
 |
Metadata Filter
 |
Reranker
 |
Relevant Sources
 |
LLM
 |
Validated Output
```

---

# 26. Agent

Não começar por aqui.

Adicionar depois que:

- engines forem confiáveis;
- context collector existir;
- tool interfaces forem estáveis;
- evals estiverem maduros.

Tools futuras:

```text
inspect_dom
get_accessibility_tree
run_engine
press_tab
press_enter
type_text
get_focus
retrieve_guideline
inspect_source
```

---

# 27. O que desenvolver primeiro

Ordem recomendada:

```text
01. Repository foundation

02. Core domain

03. CLI

04. Browser runner

05. Axe adapter

06. Finding normalizer

07. Terminal reporter

08. Context collector

09. AI provider interface

10. Structured AI output

11. Golden dataset

12. Eval runner

13. Native rules engine

14. RAG

15. Keyboard engine

16. Accessibility tree engine

17. GitHub Action

18. Source code analysis

19. VS Code

20. Accessibility Agent
```

---

# 28. Primeira feature recomendada

Começar com:

```text
scan URL + normalize axe findings
```

Não adicionar IA na primeira semana lógica do projeto.

Primeiro garantir que:

```text
URL
 -> browser
 -> engine
 -> internal domain
 -> report
```

funciona bem.

Depois adicionar IA.

Isso evita construir IA em cima de uma base instável.

---

# 29. Princípios para agentes de desenvolvimento

O agente de código deve:

- ler specs antes de codar;
- respeitar ADRs;
- alterar o mínimo necessário;
- escrever testes;
- validar lint;
- validar arquitetura;
- registrar decisões novas;
- não "melhorar" módulos não relacionados;
- não introduzir dependências sem justificativa.

---

# 30. Resultado esperado da primeira grande versão

No fim do primeiro ciclo:

```bash
a11y scan http://localhost:3000
```

deve produzir:

```text
Finding #1

Rule:
Missing accessible name

Evidence:
- axe engine detected button-name
- accessibility name is empty

Element:
<button><svg ... /></button>

AI Analysis:
This control appears to use only an icon.

Possible intent:
Favorite action

Suggested fix:
<button aria-label="Adicionar aos favoritos">

Suggestion confidence:
0.78

Human review:
Recommended
```

Esse resultado já prova o conceito completo do A11y Copilot.
