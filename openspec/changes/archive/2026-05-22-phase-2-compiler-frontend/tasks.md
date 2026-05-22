## 1. Lexer Tests

- [x] 1.1 Add unit tests to `crates/vt-lexer/src/lib.rs` for keyword tokens (`MODO`, `DETECTAR`) — assert correct `Token` variant and that keywords take priority over `Ident`
- [x] 1.2 Add unit tests for delimiter tokens (`{`, `}`, `:`) — verify they are recognized as `LBrace`, `RBrace`, `Colon`
- [x] 1.3 Add unit tests for comparator tokens (`<`, `>`, `<=`, `>=`, `==`) — verify each produces the correct `Comparator` variant
- [x] 1.4 Add unit tests for unit tokens (`m`, `cm`, `ft`, `px`) — verify each produces the correct `Unit` variant and is not swallowed by `Ident`
- [x] 1.5 Add unit tests for literals — `NumberLit` (integer `42` → `42.0`, float `2.5` → `2.5`) and `StringLit` (content without quotes)
- [x] 1.6 Add unit tests for whitespace and comment skipping — verify spaces, tabs, newlines, and `//` comments are omitted from token stream
- [x] 1.7 Add edge case test for empty input — verify empty string produces empty `Vec<Token>`
- [x] 1.8 Run `cargo test -p vt-lexer` and verify all new tests pass

## 2. LALRPOP Grammar

- [x] 2.1 Create `crates/vt-parser/src/grammar.lalrpop` with `extern` block mapping all 17 `vt_lexer::Token` variants to LALRPOP terminals
- [x] 2.2 Implement `program` production: optional `mode_decl` followed by one or more `rule_decl`, constructing `Program { mode, rules }`
- [x] 2.3 Implement `mode_decl` production: `KwModo` then `Ident(name)`, constructing `ModeDecl { name }`
- [x] 2.4 Implement `rule_decl` production: `KwDetectar` then `Ident(label)` then attributes block, constructing `DetectRule { label, attributes }`
- [x] 2.5 Implement `attribute` productions with first-token dispatch (Comparator→distance, StringLit→alert, NumberLit→priority)
- [x] 2.6 Implement `value` dispatch via LALRPOP alternatives
- [x] 2.7 Implement `distance_expr` production: `Comparator` then `NumberLit` then `Unit`, constructing `DistanceExpr`
- [x] 2.8 Add `use` statements importing `vt_core::*` and `vt_lexer::Token` at the top of the grammar file
- [x] 2.9 Run `cargo build -p vt-parser` — zero conflicts, builds clean

- [x] 3.1 Uncomment and import `lalrpop_mod!(pub grammar);` with `use lalrpop_util::lalrpop_mod`
- [x] 3.2 Define `VtParseError` struct with `message: String`, `line: usize`, `column: usize`
- [x] 3.3 Replace placeholder `parse()` with working implementation using `tokenize_with_locations` and `ProgramParser::new().parse()`
- [x] 3.4 Update public API: `pub fn parse(source: &str) -> Result<Program, Vec<VtParseError>>`
- [x] 3.5 Build verified — `cargo build -p vt-parser` compiles cleanly

## 4. Test Fixtures and Integration Tests

- [x] 4.1 Create `crates/vt-parser/tests/fixtures/valid/` directory — see `design.md` D4 for fixture organization
- [x] 4.2 Create `crates/vt-parser/tests/fixtures/valid/example_full.vt` — the canonical example from `docs/PRD.md` section 5.1 (mode + 3 rules with all attribute types)
- [x] 4.3 Create `crates/vt-parser/tests/fixtures/valid/minimal_no_mode.vt` — single `DETECTAR` rule without `MODO` declaration
- [x] 4.4 Create `crates/vt-parser/tests/fixtures/valid/all_comparators.vt` — one rule per comparator (`<`, `>`, `<=`, `>=`, `==`)
- [x] 4.5 Create `crates/vt-parser/tests/fixtures/valid/all_units.vt` — one rule per unit (`m`, `cm`, `ft`, `px`)
- [x] 4.6 Create `crates/vt-parser/tests/fixtures/valid/whitespace_comments.vt` — file with heavy whitespace, indentation, and `//` comments interspersed
- [x] 4.7 Create `crates/vt-parser/tests/fixtures/invalid/` directory and add malformed `.vt` files: missing brace, unknown attribute key, missing value, empty file

## 5. Parser Integration Tests

- [x] 5.1 Add `#[test]` in `crates/vt-parser/tests/` (or `src/lib.rs`) that parses each valid fixture and asserts `parse().is_ok()`
- [x] 5.2 Add `insta::assert_debug_snapshot!` test for the canonical `example_full.vt` — verify the AST structure matches the expected tree from `specs/vt-parser/spec.md` "Parse complete example" scenario — see `design.md` D5 for snapshot testing rationale
- [x] 5.3 Add snapshot test for `minimal_no_mode.vt` — verify `Program.mode` is `None`
- [x] 5.4 Add test for each invalid fixture asserting that `parse()` returns `Err` with at least one `ParseError`
- [x] 5.5 Add test verifying parser produces multiple errors when multiple issues exist (if LALRPOP supports this via error recovery)
- [x] 5.6 Run `cargo test -p vt-parser`, review snapshots with `cargo insta review`, and accept correct ones

## 6. CLI Pipeline Wiring

- [x] 6.1 In `crates/vt-cli/src/main.rs`, read the input file using `std::fs::read_to_string` — `crates/vt-cli/`
- [x] 6.2 Call `vt_lexer::tokenize(&source)` to get the token stream
- [x] 6.3 Call `vt_parser::parse(&source)` passing the source string directly (parser internally uses lexer) — see `design.md` D3 for the `parse()` API
- [x] 6.4 On success, print AST with `println!("{:#?}", program)` and exit 0 — see `specs/compiler-pipeline/spec.md` Requirement: "CLI executes the lex → parse pipeline"
- [x] 6.5 On parse error, print each error with line/column and exit with non-zero code — see `specs/compiler-pipeline/spec.md` Requirement: "Pipeline stops on parse error"
- [x] 6.6 Run `cargo build -p vt-cli` and test end-to-end: `cargo run -p vt-cli -- compile docs/PRD.md` (for the example embedded in PRD) — verify Debug AST output

## 7. Quality Assurance

- [x] 7.1 Run `cargo fmt --all` and verify no formatting changes needed
- [x] 7.2 Run `cargo clippy --workspace -- -D warnings` and fix any warnings
- [x] 7.3 Run `cargo test --workspace` and verify all tests pass (including pre-existing vt-semantics test)
- [x] 7.4 Commit with message: `feat(vt-parser): implement LALRPOP grammar and parser with snapshot tests`
