## Why

Phase 1 scaffolding is complete. The monorepo has 6 Rust crates with dependency wiring, both runtimes are initialized, and the AST types (`vt-core`) and token definitions (`vt-lexer`) are fully implemented. However, the parser — the bridge between tokens and AST — is a stub. Without it, no `.vt` file can be compiled. Phase 2 implements the compiler frontend (lexer → parser → AST) to produce a validated parse tree from any valid `.vt` input.

## What Changes

- **New**: `crates/vt-parser/src/grammar.lalrpop` — LALRPOP grammar file mapping all 17 token types to the 8 EBNF productions defined in `docs/grammar.md`, with Rust action code constructing `vt_core::Program` AST nodes
- **Modified**: `crates/vt-parser/src/lib.rs` — uncomment `lalrpop_mod!(pub grammar)`, replace placeholder `parse()` with the generated LR(1) parser, expose a public `parse(source: &str) -> Result<Program, Vec<ParseError>>` API
- **New**: `crates/vt-parser/src/` — integration tests using `insta` snapshot testing against the canonical example `.vt` from `docs/PRD.md` section 5.1, plus negative test cases for malformed input
- **New**: `crates/vt-lexer/` — unit tests for individual token recognition (keywords, literals, comparators, units, edge cases like comments and whitespace)
- **New**: `crates/vt-cli/src/main.rs` — wire the pipeline: read `.vt` file → lex → parse → print AST (semantics and codegen remain stubs for Phase 3+)

No changes to `vt-core` (AST types are complete), `vt-semantics`, or `vt-codegen`. No changes to either runtime.

## Capabilities

### New Capabilities

- **vt-parser**: LALRPOP-based LR(1) parser that transforms a sequence of `vt_lexer::Token` into a `vt_core::Program` AST. Must parse the full EBNF grammar (mode declarations, detect rules with distance/alert/priority attributes, optional mode, required rules). Must produce structured parse errors with line/column information.
- **compiler-pipeline**: End-to-end flow from `.vt` source file to printed AST via the CLI. The `vt` binary reads a file, invokes `vt_lexer::tokenize`, passes tokens to `vt_parser::parse`, and prints the resulting AST (Debug format). Validates that all Phase 1 crate wiring is correct.

### Modified Capabilities

None. No existing specs or capabilities are being modified.

## Impact

- **Crates affected**: `vt-parser` (primary — grammar + integration), `vt-cli` (secondary — pipeline wiring)
- **Crates consumed but unchanged**: `vt-core` (AST types used as target), `vt-lexer` (tokens used as input)
- **Build system**: LALRPOP build script already exists in `vt-parser/build.rs` — no changes needed
- **Testing**: New `insta` snapshots directory, new integration test `.vt` fixture files
- **Dependencies**: All already declared in workspace `Cargo.toml` (`lalrpop 0.20`, `lalrpop-util 0.20`, `insta 1.39`, `logos 0.14`)
- **Runtimes**: No impact — desktop and mobile runtimes remain in scaffolded state
