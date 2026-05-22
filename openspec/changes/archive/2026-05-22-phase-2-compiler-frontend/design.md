## Context

Phase 1 established the monorepo structure, created all 6 Rust crates, and fully implemented the AST types (`vt-core`) and token definitions (`vt-lexer`). The `vt-parser` crate has a working `build.rs` that conditionally invokes LALRPOP, but the grammar file (`src/grammar.lalrpop`) does not exist, and the `parse()` function is a placeholder.

The EBNF grammar is fully specified in `docs/grammar.md` with 8 productions: `program`, `modo_decl`, `rule_decl`, `attribute`, `key`, `value`, `distance_expr`, `comparator`, `unit`. The lexer's `Token` enum has 17 variants covering all terminals (keywords, delimiters, comparators, units, literals, identifiers). The AST types in `vt-core` map 1:1 to grammar non-terminals.

The `vt-cli` crate has a `clap` CLI struct but does not yet call any compiler phases.

## Goals / Non-Goals

**Goals:**
- Create a complete `grammar.lalrpop` that maps all 17 `vt_lexer::Token` variants to the 8 EBNF productions
- Implement action code that constructs `vt_core::Program` AST nodes during parsing
- Replace the placeholder `parse()` function with the generated LR(1) parser
- Add lexer unit tests validating correct tokenization of all keywords, literals, comparators, and units
- Add parser integration tests with `insta` snapshot testing against the canonical example `.vt`
- Wire the `vt-cli` binary to read a `.vt` file, lex it, parse it, and print the AST

**Non-Goals:**
- Semantic analysis (Phase 3 — `vt-semantics`)
- Code generation to Python or JavaScript (Phase 4+ — `vt-codegen`)
- Error recovery in the parser (best-effort error messages only)
- Runtime engine implementation
- LALRPOP grammar macros beyond the standard `extern` token mapping

## Decisions

### D1: Use LALRPOP extern block for token mapping (not a custom Token type wrapper)

**Chosen**: Map each `vt_lexer::Token` variant to a LALRPOP terminal directly in the `extern { }` block. The parser action code receives `Token::KwModo`, `Token::KwDetectar`, etc. as typed values.

**Rationale**: LALRPOP's `extern` block is purpose-built for this. It avoids an intermediate token conversion layer. The lexer already produces the exact enum variants LALRPOP expects.

**Alternative considered**: Wrapping `Token` in a newtype with `From` impls. Rejected — adds indirection with no benefit since the lexer's `Token` type is stable and owned by `vt-lexer`.

### D2: Map attribute keys ("distancia", "alerta", "prioridad") via Ident string matching in parser actions

**Chosen**: The `key` production matches `Ident(s)` and the action code does a `match` on `s.as_str()` to decide the `Attribute` variant. Unknown keys produce a parse error.

**Rationale**: `distancia`, `alerta`, and `prioridad` are not reserved keywords — they are attribute names within `DETECTAR` blocks. Making them separate tokens would pollute the lexer with domain-specific strings. The parser is the right layer for this semantic distinction.

**Alternative considered**: Adding `KwDistancia`, `KwAlerta`, `KwPrioridad` tokens. Rejected — these aren't language keywords, they're attribute keys. If more attributes are added later, the lexer shouldn't need changes.

### D3: Use `Result<Program, Vec<ParseError>>` as the parse return type

**Chosen**: Return multiple errors (not just the first one) so the user can fix all issues in one pass.

**Rationale**: LALRPOP's default error recovery supports collecting multiple errors. A `Vec<ParseError>` with line/column and message aligns with the Rust ecosystem convention (similar to `rustc`'s multi-error output) and the project's error message convention.

**Alternative considered**: `Result<Program, String>`. Rejected — single error messages force iterative fix cycles, which is poor UX for a DSL compiler.

### D4: Store `.vt` test fixtures alongside parser tests, not in a separate directory

**Chosen**: Put `.vt` fixture files in `crates/vt-parser/tests/fixtures/` and reference them from integration tests via `include_str!()` or `std::fs::read_to_string`.

**Rationale**: Colocating test data with tests makes it obvious what fixture belongs to which test. `cargo test` automatically finds tests in `tests/`.

### D5: Lexer tests use Rust unit tests with `#[test]`, parser tests use `insta` snapshots

**Chosen**: Lexer tokenization is deterministic — simple `assert_eq!` is sufficient. Parser output is a complex AST tree — `insta::assert_debug_snapshot!` provides human-readable diff review.

**Rationale**: Snapshot testing for AST output catches regressions in tree shape that would be tedious to assert manually. Lexer tests are short and self-contained.

## Risks / Trade-offs

- **[Risk] LALRPOP grammar conflicts**: The `key` production matching `Ident` for both attribute names and the `MODO`/`DETECTAR` label identifiers could create shift/reduce ambiguities. → **Mitigation**: LALRPOP reports conflicts at build time. The grammar design separates `modo_decl` (which consumes `Ident` for the mode name) from `rule_decl` (which consumes `Ident` for the detect label, then `{`, then attributes). The `key` in attributes is unambiguous because it's preceded by `:` and followed by another `:`. A `macro` rule may be needed to handle the grammar structure correctly.
- **[Risk] Token priority in logos**: `Ident` matches any `[a-zA-Z_][a-zA-Z0-9_]*` which theoretically could swallow `MODO`, `DETECTAR`, and unit strings (`m`, `cm`, `ft`, `px`). → **Mitigation**: Already handled — `logos` prioritizes `#[token(...)]` (priority 0) over `#[regex(...)]` (priority 1+). Verified by existing `vt-lexer` code.
- **[Risk] Number literal ambiguity**: `NumberLit` matches integers AND floats. The grammar must ensure `distance_expr` only accepts `NumberLit` after a `Comparator`. → **Mitigation**: LALRPOP's LR(1) parsing handles this naturally since `NumberLit` appears only in specific productions.
- **[Trade-off] No error recovery**: The parser stops at the first parse error for now. LALRPOP supports error recovery tokens (`!` and `?`) but these add complexity. → **Accepted**: Phase 2 delivers a working parser. Error recovery can be added in Phase 3 alongside semantic analysis when error UX becomes critical.

## Open Questions

- Should the parser accept trailing commas in attribute lists? (Not in current grammar, but common in config DSLs for copy-paste friendliness)
- Should duplicate attributes in the same rule be a parse error or deferred to semantic analysis? (Recommendation: defer to Phase 3 — the parser should be lenient, semantics strict)
