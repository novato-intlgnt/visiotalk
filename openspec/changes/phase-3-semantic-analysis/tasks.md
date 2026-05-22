## 1. Prerequisites — AST span metadata & error types

- [ ] 1.1 Check if `vt_core` AST types (`DetectRule`, `DistanceExpr`, `Attribute::Alert`) carry line/column span fields; add `span: Option<Span>` if missing — file `crates/vt-core/src/lib.rs`
- [ ] 1.2 Update parser action code in `vt-parser` to populate span fields during AST construction — file `crates/vt-parser/src/grammar.lalrpop`
- [ ] 1.3 Add `Severity` enum (`Error`, `Warning`) and expand `SemanticError` struct with `severity`, `suggestion: Option<String>` fields in `vt-semantics` — file `crates/vt-semantics/src/lib.rs`

## 2. Core semantic validations

- [ ] 2.1 Define YOLOv8-nano COCO label vocabulary as a `const YOLO_LABELS: &[&str]` array in `vt-semantics` — file `crates/vt-semantics/src/lib.rs`
- [ ] 2.2 Implement empty program validation (at least one `DETECTAR` rule → Error) — file `crates/vt-semantics/src/lib.rs`
- [ ] 2.3 Implement label vocabulary validation (unknown labels → Warning with suggestion of valid labels) — file `crates/vt-semantics/src/lib.rs`
- [ ] 2.4 Implement distance range validation (≤ 0 → Error, ≥ 100 → Warning) — file `crates/vt-semantics/src/lib.rs`
- [ ] 2.5 Implement priority collision detection (duplicate priorities → Warning identifying colliding rule labels) — file `crates/vt-semantics/src/lib.rs`
- [ ] 2.6 Implement alert text validation (empty → Warning, >200 chars → Warning) — file `crates/vt-semantics/src/lib.rs`
- [ ] 2.7 Refactor `analyze()` to run all validations in a single pass collecting all issues into `Vec<SemanticError>` — file `crates/vt-semantics/src/lib.rs`

## 3. CLI pipeline integration

- [ ] 3.1 Wire `vt_semantics::analyze()` into the `vt-cli` compile command after `vt_parser::parse()` — file `crates/vt-cli/src/main.rs`
- [ ] 3.2 Print semantic errors/warnings to stderr with line/column and severity; exit non-zero on `Err` result — file `crates/vt-cli/src/main.rs`

## 4. Testing

- [ ] 4.1 Write unit tests for each validation: empty program, unknown label, valid label, negative distance, zero distance, valid distance, large distance, duplicate priorities, unique priorities, empty alert, long alert, valid alert — file `crates/vt-semantics/src/lib.rs`
- [ ] 4.2 Write integration tests with `insta` snapshots: lex → parse → analyze for valid and invalid `.vt` fixtures — file `crates/vt-semantics/tests/semantic_tests.rs`
- [ ] 4.3 Create `.vt` fixture files for integration tests: valid program, invalid distances, unknown labels, duplicate priorities, empty/missing alerts — directory `crates/vt-semantics/tests/fixtures/`
- [ ] 4.4 Run `cargo test` on the full workspace and verify all tests pass — workspace root
