## Context

The lexer (`vt-lexer`) and parser (`vt-parser`) from Phase 2 produce a validated AST (`vt_core::Program`) from `.vt` source files. However, the AST can contain logically invalid configurations that the parser cannot detect: negative distances, unknown YOLO labels, conflicting priorities, or empty alert strings. These must be caught before code generation (Phase 4) to prevent silent runtime failures.

The `vt-semantics` crate currently has a placeholder implementation that only checks for empty rule lists. All semantic validation rules are specified in `docs/grammar.md` section "Restricciones del Analizador Semántico" — a table of 6 validations with error/warning severity and Spanish-language messages.

The semantic analyzer operates on an already-parsed `&Program` and produces either `Ok(())` (valid program) or `Err(Vec<SemanticError>)` (one or more issues found). Warnings are included in the error vector alongside errors — the caller decides whether to treat warnings as blocking.

## Goals / Non-Goals

**Goals:**
- Implement all 6 validations from `docs/grammar.md` semantics table
- Categorize each validation as Error (blocks compilation) or Warning (does not block)
- Attach line/column location to every error/warning using AST span metadata
- Provide actionable suggestion text in error messages (e.g., "Etiquetas válidas: persona, coche, ...")
- Integrate `vt-semantics` into the `vt-cli` pipeline (lex → parse → analyze)
- Write integration tests covering the full pipeline with snapshots

**Non-Goals:**
- Code generation (Phase 4 — `vt-codegen`)
- Loading YOLO label vocabulary dynamically from a config file (hardcoded list for Phase 3)
- Error recovery — the analyzer collects all issues in one pass but does not attempt to fix and continue
- Cross-platform validation differences (same rules apply to desktop and mobile)

## Decisions

### D1: Single-pass validation collecting all errors at once

**Chosen**: The `analyze()` function walks the AST once and accumulates all errors and warnings into a `Vec<SemanticError>`, returning them all together. No early exit on first error.

**Rationale**: Unlike the parser, semantic errors are independent of each other — a bad distance on rule 1 doesn't prevent validating rule 2's label. Returning all issues at once provides better UX, letting the user fix everything in one iteration. This matches the `rustc` multi-error model.

**Alternative considered**: Fail-fast (return first error only). Rejected — forces iterative fix cycles with no benefit for independent validations.

### D2: Hardcoded YOLOv8-nano label vocabulary

**Chosen**: Define a `const YOLO_LABELS: &[&str]` array containing the 80 COCO class names that YOLOv8-nano recognizes. The semantic analyzer checks each `DetectRule.label` against this list.

**Rationale**: The label vocabulary is a property of the AI model (YOLOv8-nano), not the user's `.vt` file. Hardcoding aligns with the "purely declarative DSL" philosophy — users shouldn't need to know COCO class IDs. The const array is a single source of truth. When model upgrades happen (e.g., YOLOv9), updating one array updates all validation.

**Alternative considered**: Load labels from `models/labels.txt` at compile time via `include_str!()`. Rejected — adds I/O dependency and the labels file doesn't exist yet (models are gitignored). Can migrate to this later.

### D3: Warnings and errors share the same return type with a severity enum

**Chosen**: Add a `severity: Severity` field to `SemanticError` with variants `Error` and `Warning`. The `analyze()` function returns `Result<(), Vec<SemanticError>>` — it returns `Err` if ANY error exists (warnings alone don't fail). The CLI can decide whether `--strict` mode treats warnings as errors.

**Rationale**: Distinguishing severity is essential — priority collisions and unknown labels should warn but not block compilation, while negative distances and empty programs must block. A single `Vec` ordered by source location keeps the API simple for both integration tests and CLI reporting.

**Alternative considered**: Separate `Vec<Error>` and `Vec<Warning>` return types. Rejected — complicates the API, requires sorting by location for display, and duplicates error struct fields.

### D4: AST nodes must carry span (line/column) metadata for error reporting

**Chosen**: If `vt_core` types don't already have span fields, add optional `span: Option<Span>` to `DetectRule` and `DistanceExpr`. The parser populates these during AST construction. The semantic analyzer reads spans to attach location to error messages.

**Rationale**: Error messages without file locations are useless in a compiler. The grammar already specifies "El analizador semántico debe..." for most rules, implying location-aware reporting. If Phase 2 didn't add spans, this phase adds them as a prerequisite change to `vt-core` and `vt-parser`.

**Alternative considered**: Re-lexing the source to find token positions. Rejected — fragile, duplicates parser work, and loses AST context.

### D5: Integration tests use snapshot testing across the full pipeline

**Chosen**: Write tests in `vt-semantics/tests/` that call `lex → parse → analyze` and snapshot the result (either the program or the error list). Use `insta::assert_debug_snapshot!` for the full error output.

**Rationale**: The integration tests in Phase 2 (parser snapshots) established the pattern. Semantic tests follow the same convention — snapshot diffs make regression detection trivial and let reviewers verify error message quality.

## Risks / Trade-offs

- **[Risk] Span fields may not exist in vt-core yet**: If Phase 2 didn't add file location metadata to AST nodes, this phase must backfill them. → **Mitigation**: The first task in this phase checks and adds `Span` fields if missing. This is a minimal, non-breaking change (optional fields, `#[serde(default)]`).

- **[Risk] Label vocabulary may diverge from actual model**: If the hardcoded YOLO_LABELS array goes out of sync with the deployed model, the compiler will reject valid labels. → **Mitigation**: The vocabulary matches COCO 2017 classes, which are stable. A comment linking to the Ultralytics source ensures traceability. Phase 9 (testing) validates end-to-end with the actual model.

- **[Risk] Spanish error messages may need i18n later**: Hardcoding Spanish messages works for the initial user base but could create maintenance debt. → **Accepted**: i18n is out of scope. The messages are simple and few (6 validations). If needed, they can be extracted to a messages module later.

- **[Trade-off] Priority collision detection is O(n²)**: Comparing every rule's priority against every other rule's priority is quadratic. → **Accepted**: `.vt` files typically have <20 rules. O(n²) with n≤20 is negligible. Optimization is premature.

## Open Questions

- Should `analyze()` also check for duplicate `DETECTAR` labels on the same object type? (Not in current grammar spec, but potentially confusing for users.)
- Should distance value normalization (converting `cm` and `ft` to `m`) happen during semantic analysis or code generation? (Recommendation: codegen — semantics validates, codegen transforms.)
