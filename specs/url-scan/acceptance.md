# Acceptance Criteria: URL Accessibility Scan (`url-scan`)

## Scenario 1: Detecting icon-only buttons without accessible names

**Given** a web page (`fixtures/ambiguous-buttons.html`) containing:
- `<button id="fav-btn"><svg aria-hidden="true">...</svg></button>` inside a product card
- `<button id="like-btn"><svg aria-hidden="true">...</svg></button>` inside a social post
- `<button id="buy-btn">Comprar agora</button>` with visible text

**When** the page is scanned

**Then**:
- The system creates deterministic `Finding` entries for `#fav-btn` and `#like-btn` with category `MissingAccessibleName` and severity `Serious` or `Critical`.
- No finding is created for `#buy-btn`.
- Each finding includes deterministic `Evidence` from the engine (`rule_id: "button-name"`).
- Any future AI suggestion (e.g. `"Adicionar aos favoritos"` vs. `"Curtir publicação"`) is stored strictly under `interpretation`, never as deterministic `evidence`.

---

## Scenario 2: Rejecting an invalid target URL

**Given** an empty target `""` or an unsupported scheme `"ftp://example.com"`

**When** `ScanTarget::new` is called

**Then**:
- The system returns a `DomainError` before spawning any subprocess or browser.

---

## Scenario 3: Rejecting a finding without deterministic evidence

**Given** an attempt to construct a `Finding` with an empty `evidence` vector

**When** `Finding::new` is called

**Then**:
- The constructor returns `Err(DomainError::MissingDeterministicEvidence)`, enforcing ADR 0002 at the type boundary.
