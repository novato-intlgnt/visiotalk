## 1. Module setup

- [x] 1.1 Create `crates/vt-cli/src/debug.rs` with empty `CompilerDebugger` struct and `new()` constructor — file `crates/vt-cli/src/debug.rs`
- [x] 1.2 Declare `mod debug;` in `crates/vt-cli/src/main.rs` and add `use` import — file `crates/vt-cli/src/main.rs`
- [x] 2.1 Add `#[arg(long, default_value_t = false)] debug: bool` to the `Compile` subcommand in clap derive struct — file `crates/vt-cli/src/main.rs`
- [x] 2.2 Refactor pipeline in `main()` to extract source reading into a variable (read once, pass to all stages) — file `crates/vt-cli/src/main.rs`
- [x] 2.3 Implement stage-based pipeline loop: define stages as a sequence of `(header, fn)` and iterate, calling `CompilerDebugger` methods between stages when `--debug` is active. Ensure the loop structure accommodates future phases (Phase 3 semantic analysis will insert as another stage). — file `crates/vt-cli/src/main.rs`
- [x] 2.4 Verify that without `--debug`, behavior is identical to current (no extra output, no extra allocations). Run `cargo run -- compile <existing_test.vt>` and confirm output matches.

## 3. CompilerDebugger — Phase 1 & 2

- [x] 3.1 Implement `CompilerDebugger::print_source(&self, source: &str, filepath: &str)` — prints `=== FASE 1: CÓDIGO FUENTE (<filepath>) ===`, raw source, blank line — file `crates/vt-cli/src/debug.rs`
- [x] 3.2 Implement `CompilerDebugger::print_tokens(&self, tokens: &[(usize, Token, usize)])` — prints `=== FASE 2: TOKENS (<count> tokens) ===` followed by numbered lines in format `<idx>: <VariantName> "<lexeme>"`. Use a helper to extract the variant name from `Token` (e.g., via `std::fmt::Debug` or a manual `match`). For 0 tokens, print only the header with count. — file `crates/vt-cli/src/debug.rs`
- [x] 4.1 Implement helper `fn ast_display(&self, program: &Program) -> String` that builds the tree string using recursive `format_node(node, prefix: &str, is_last: bool)` pattern. The recursive function uses `├── `, `└── `, `│   ` for tree connectors. — file `crates/vt-cli/src/debug.rs`
- [x] 4.2 Implement `Program` formatting: root node `Program`, child `ModeDecl` if `Some`, then each `DetectRule` as `Rule[N]: DETECTAR "label"` with attributes as children — file `crates/vt-cli/src/debug.rs`
- [x] 4.3 Implement attribute formatting inline: `Distance: <comp> <val> <unit>` (e.g., `Distance: < 2.0 m`), `Alert: "text"`, `Priority: <n>` — file `crates/vt-cli/src/debug.rs`
- [x] 4.4 Implement comparator and unit display: `<`, `>`, `<=`, `>=`, `==` for comparators; `m`, `cm`, `ft`, `px` for units — file `crates/vt-cli/src/debug.rs`
- [x] 4.5 Implement `CompilerDebugger::print_ast(&self, program: &Program)` — prints `=== FASE 3: AST ===` followed by the tree from `ast_display()` — file `crates/vt-cli/src/debug.rs`

## 5. Testing

- [x] 5.1 Create fixture directory `crates/vt-cli/tests/fixtures/` and add `sample.vt` with a complete valid program (mode, one rule with distance+alert+priority) — directory `crates/vt-cli/tests/fixtures/`
- [x] 5.2 Add `empty.vt` fixture (empty file) and `multi_rule.vt` fixture (3 rules with varied attributes) — directory `crates/vt-cli/tests/fixtures/`
- [x] 5.3 Write integration test in `crates/vt-cli/tests/debug_snapshots.rs` that runs the binary with `--debug` on each fixture, captures stdout, and uses `insta::assert_snapshot!`. Test both `--debug` and no-`--debug` cases (the latter asserting identical output to pre-change behavior). — file `crates/vt-cli/tests/debug_snapshots.rs`
- [x] 5.4 Run `cargo test` on the full workspace and verify all tests pass including new snapshots — workspace root
- [x] 6.1 Run `cargo fmt` and `cargo clippy -- -D warnings` on the workspace. Fix any warnings or errors.
- [x] 6.2 Verify modularity: `grep -r "println\|eprintln\|dbg!" crates/vt-lexer/src/ crates/vt-parser/src/ crates/vt-core/src/ crates/vt-semantics/src/` shows no debug-specific printing in those crates.
- [x] 6.3 Manual smoke test: compile 2–3 `.vt` files with and without `--debug`, verify output format matches spec, verify exit codes.
