# A11y Copilot

## Visão do projeto

**A11y Copilot** é uma ferramenta open source, Rust-first, para incorporar acessibilidade diretamente ao fluxo de desenvolvimento de aplicações web.

O objetivo não é "certificar" automaticamente que um site é acessível. O objetivo é ajudar desenvolvedores a:

- detectar problemas de acessibilidade;
- entender por que eles importam;
- relacionar problemas a critérios e práticas de acessibilidade;
- receber sugestões de correção contextualizadas;
- gerar evidências técnicas para revisão humana;
- prevenir regressões de acessibilidade durante o desenvolvimento.

A proposta central é combinar:

1. **análise determinística**;
2. **motores de acessibilidade plugáveis**;
3. **coleta de contexto da interface**;
4. **IA contextual e fundamentada**;
5. **avaliação contínua da própria IA**.

---

# Princípios

## 1. IA não é a única fonte de verdade

Problemas determinísticos devem ser detectados por mecanismos determinísticos.

A IA entra principalmente para:

- interpretar contexto;
- explicar impacto;
- sugerir correções;
- priorizar;
- relacionar evidências;
- lidar com ambiguidades;
- auxiliar revisão humana.

---

## 2. Axe-core é um motor, não a fundação do produto

O projeto pode usar axe-core inicialmente, mas o domínio do A11y Copilot não deve depender diretamente das estruturas do axe.

Arquitetura conceitual:

```text
                 A11y Copilot Core
                        |
              Analysis Orchestrator
                        |
       +----------------+----------------+
       |                |                |
       v                v                v

 Deterministic      Behavioral        Semantic
   Engines            Engines          Engines

       |                |                |

   axe-core          keyboard            AI
 native rules         focus             RAG
                     navigation         context
```

Cada engine produz um formato interno comum de `Finding`.

Isso permite substituir, combinar e criar novos motores no futuro.

---

## 3. Rust-first, não necessariamente 100% Rust

Rust será utilizado para:

- CLI;
- core de domínio;
- orchestration;
- normalização de findings;
- rules engine;
- concorrência;
- cache;
- relatórios;
- integração com IA;
- RAG;
- evals;
- observabilidade.

Onde o ecossistema web já possui ferramentas maduras, elas podem ser usadas por adapters.

Exemplo inicial:

```text
Rust Core
   |
   +-- Browser Adapter
           |
           +-- Node.js
                 |
                 +-- Playwright
                 +-- axe-core
```

---

# Stack inicial

## Core

- Rust
- Cargo Workspaces
- Tokio
- Serde
- serde_json
- Reqwest
- Clap
- thiserror
- anyhow
- tracing

## Browser/runtime

- Node.js
- Playwright
- axe-core

## Testes

- cargo test
- snapshot testing
- Playwright Test
- fixtures de páginas acessíveis e inacessíveis

## IA

O domínio deve ser agnóstico ao provedor.

Possíveis providers:

- OpenAI
- Gemini
- Anthropic
- modelos locais

Interface conceitual:

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

# Estrutura proposta

```text
a11y-copilot/
|
|-- Cargo.toml
|
|-- crates/
|   |-- cli/
|   |-- core/
|   |-- scanner/
|   |-- rules/
|   |-- ai/
|   |-- retrieval/
|   |-- evals/
|   |-- reporters/
|   `-- engines/
|       |-- axe/
|       |-- native/
|       |-- keyboard/
|       `-- accessibility-tree/
|
|-- browser/
|   `-- playwright-runner/
|
|-- fixtures/
|
|-- evals/
|
|-- specs/
|
|-- docs/
|   `-- adr/
|
`-- examples/
```

---

# Modelo de domínio

Um `Finding` deve pertencer ao A11y Copilot, não ao axe-core ou a qualquer outra ferramenta.

Exemplo conceitual:

```rust
pub struct Finding {
    pub id: FindingId,
    pub category: Category,
    pub severity: Severity,
    pub element: ElementContext,
    pub evidence: Vec<Evidence>,
    pub source: FindingSource,
    pub confidence: Confidence,
}
```

Um finding pode agregar múltiplas evidências:

```text
Finding: botão sem nome acessível

Evidence:
- axe-core detectou ausência de nome
- accessibility tree possui role=button e name=""
- regra nativa confirmou ausência de nome
- IA inferiu que o ícone provavelmente representa "favoritos"
```

---

# Pipeline principal

```text
Target URL
   |
   v
Browser Adapter
   |
   v
Accessibility Engines
   |
   v
Raw Findings
   |
   v
Normalizer
   |
   v
Context Collector
   |
   v
Evidence Aggregator
   |
   v
AI Analyzer
   |
   v
Validation
   |
   v
Final Finding
   |
   v
Reporter
```

---

# MVP

O objetivo do MVP é provar o pipeline completo sem construir ainda um agente complexo.

## Escopo

Comando:

```bash
a11y scan http://localhost:3000
```

O MVP deve:

- abrir uma URL;
- executar análise automatizada;
- coletar findings;
- converter findings para o modelo interno;
- coletar contexto relevante;
- enriquecer findings com IA;
- gerar relatório de terminal;
- gerar relatório JSON;
- permitir execução local;
- possuir testes;
- possuir dataset inicial de evals.

## Engines do MVP

### Axe Engine

Primeiro adapter externo.

Responsável por:

- executar axe-core;
- receber resultados;
- converter o resultado para o domínio do A11y Copilot.

### Native Engine

Começa pequeno.

Primeiras regras possíveis:

- botão sem nome acessível;
- imagem sem alternativa;
- input sem label associado;
- heading hierarchy problemática;
- elementos interativos sem semântica adequada.

Não precisa competir com axe-core. Serve para construir nosso próprio conhecimento e engine.

---

# IA no MVP

A IA recebe apenas contexto controlado.

Exemplo:

```text
Finding
+ elemento afetado
+ HTML relevante
+ texto próximo
+ role
+ accessible name
+ evidências
+ metadados da regra
```

A saída deve ser estruturada.

Exemplo:

```text
explanation
affected_users
suggested_fix
confidence
requires_human_review
reasoning_category
```

A IA deve distinguir entre:

## Fato

```text
O botão não possui nome acessível.
```

## Interpretação

```text
O ícone sugere que a ação pode significar "Adicionar aos favoritos".
```

Essa distinção deve fazer parte do design do produto.

---

# Evals desde o início

O A11y Copilot deve seguir **Evaluation-Driven Development** para features de IA.

Estrutura:

```text
evals/
|-- button-name/
|-- alt-text/
|-- form-label/
|-- keyboard/
|-- modal/
`-- aria/
```

Métricas possíveis:

- correctness;
- grounding;
- fix usefulness;
- hallucination rate;
- false-positive rate;
- confidence calibration;
- human-review accuracy.

Comando futuro:

```bash
a11y eval
```

Resultado:

```text
Accessibility AI Evaluation

Cases:                   380
Detection accuracy:      97.8%
WCAG grounding:          94.2%
Fix correctness:         91.5%
Hallucination rate:       2.1%
```

---

# Pós-MVP

## Fase 2 - RAG

Criar base de conhecimento sobre:

- WCAG;
- WAI-ARIA Authoring Practices;
- MDN Accessibility;
- documentação de frameworks;
- documentação interna de rules.

Fluxo:

```text
Finding
   |
Query Builder
   |
Retrieval
   |
Reranking
   |
Relevant Evidence
   |
LLM
```

---

## Fase 3 - Keyboard Engine

Testar:

- TAB;
- SHIFT+TAB;
- ENTER;
- SPACE;
- foco;
- focus trap;
- elementos inalcançáveis;
- ordem inesperada de foco.

---

## Fase 4 - Accessibility Tree Engine

Analisar:

- role;
- accessible name;
- value;
- state;
- relacionamentos.

---

## Fase 5 - Source Code Analysis

Suporte progressivo para:

- React;
- Angular;
- Vue;
- HTML/templates.

Objetivos:

- relacionar DOM com source code;
- sugerir correções diretamente no componente;
- gerar patches;
- gerar testes.

---

## Fase 6 - GitHub Action

Uso:

```yaml
- uses: a11y-copilot/action@v1
```

Funcionalidades:

- rodar em pull requests;
- detectar novas regressões;
- comentar findings;
- gerar summary;
- falhar pipeline conforme política configurada.

---

## Fase 7 - Pull Request Reviewer

Exemplo:

```text
A11y Copilot

3 new accessibility issues detected.

src/components/Header.tsx

Search button does not have an accessible name.

Evidence:
- accessibility tree name is empty
- deterministic rule confirmed

Suggested fix:
<button aria-label="Pesquisar">
```

---

## Fase 8 - VS Code Extension

Possibilidades:

- findings inline;
- quick fixes;
- explicações;
- execução local;
- comandos para abrir documentação;
- sugestões fundamentadas.

---

## Fase 9 - Accessibility Agent

Objetivo:

```text
"Verifique se é possível concluir uma compra usando apenas o teclado."
```

Tools possíveis:

```text
inspect_dom
get_accessibility_tree
run_accessibility_engine
press_tab
press_enter
type_text
get_focused_element
retrieve_wcag
inspect_source_code
```

O agente executa fluxos e produz evidências.

---

# Futuro

A visão de longo prazo é transformar o A11y Copilot em uma plataforma open source de accessibility engineering.

Possíveis extensões:

- plugin system;
- custom rules SDK;
- custom engines;
- benchmarks públicos;
- datasets de acessibilidade;
- dashboards;
- SARIF output;
- IDE integrations;
- CI/CD integrations;
- accessibility regression testing;
- visual accessibility analysis;
- cognitive accessibility analysis;
- multimodal analysis;
- support para apps mobile;
- remote/browser farm;
- organization policies.

---

# Diferencial

O A11y Copilot não deve ser:

```text
axe-core + prompt para um LLM
```

Ele deve ser:

```text
deterministic accessibility analysis
+
behavioral analysis
+
context collection
+
evidence aggregation
+
grounded AI reasoning
+
evaluation-driven AI engineering
+
human review
```

---

# Missão

> Make accessibility part of the developer workflow instead of an audit performed at the end of development.

---

# Posicionamento

O A11y Copilot é um **assistente de engenharia de acessibilidade**, e não uma ferramenta automática de certificação.

Ele deve ajudar equipes a construir produtos melhores desde o início, mantendo transparência sobre:

- o que foi detectado;
- como foi detectado;
- quais evidências existem;
- onde existe incerteza;
- quando revisão humana é necessária.
