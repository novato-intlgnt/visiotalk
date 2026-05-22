## Requirements

### Requirement: Debug flag activates pipeline trace
When invoked with `vt compile --debug <input>`, the compiler SHALL print the output of each pipeline phase to stdout in structured, human-readable format. Without `--debug`, the compiler SHALL behave identically to the current non-debug mode (no additional output, no performance overhead).

#### Scenario: Debug flag present
- **WHEN** running `vt compile --debug sample.vt`
- **THEN** the output SHALL include the source code, token list, and AST tree in separate labeled sections

#### Scenario: Debug flag absent
- **WHEN** running `vt compile sample.vt` without `--debug`
- **THEN** the output SHALL be identical to the current non-debug behavior (only the AST Debug representation or eventual codegen output)

### Requirement: Phase 1 prints raw source code
When `--debug` is active, the `CompilerDebugger` SHALL print the full contents of the input `.vt` file, prefixed by a header line `=== FASE 1: CÓDIGO FUENTE (<filename>) ===` and followed by a blank line separator.

#### Scenario: Print source of a valid file
- **WHEN** compiling `sample.vt` with `--debug` where the file contains `MODO test\nDETECTAR persona { distancia: < 2.0m }`
- **THEN** the output SHALL begin with `=== FASE 1: CÓDIGO FUENTE (sample.vt) ===` followed by the exact file contents and a trailing blank line

#### Scenario: Print source of an empty file
- **WHEN** compiling an empty `.vt` file with `--debug`
- **THEN** the header SHALL still print, followed by an empty body and trailing blank line

### Requirement: Phase 2 prints token list
When `--debug` is active, the `CompilerDebugger` SHALL print a numbered list of all tokens produced by the lexer. Each token SHALL be displayed on its own line in the format `<index>: <TokenVariant> "<lexeme>"` where index is 1-based, TokenVariant is the Rust enum variant name (e.g., `KwModo`, `Ident`, `NumberLit`), and lexeme is the token's Display representation. The list SHALL be prefixed by `=== FASE 2: TOKENS (<count> tokens) ===`.

#### Scenario: Print tokens for a simple program
- **WHEN** compiling `DETECTAR persona { }` with `--debug`
- **THEN** Phase 2 output SHALL list tokens: `1: KwDetectar "DETECTAR"`, `2: Ident "persona"`, `3: LBrace "{"`, `4: RBrace "}"` in that order

#### Scenario: Print tokens for empty source
- **WHEN** compiling an empty file with `--debug`
- **THEN** Phase 2 output SHALL show `=== FASE 2: TOKENS (0 tokens) ===` with no token lines

### Requirement: Phase 3 prints AST as visual tree
When `--debug` is active, the `CompilerDebugger` SHALL print the parsed AST using Unicode tree-drawing characters (`├── `, `└── `, `│   `). Program SHALL appear as the root node, with child nodes (ModeDecl, DetectRules) rendered as branches. Attributes within rules SHALL be rendered as child branches with their values inline. The tree SHALL be prefixed by `=== FASE 3: AST ===`.

#### Scenario: Print AST for a complete program
- **WHEN** compiling a program with mode, one rule with distance, alert, and priority with `--debug`
- **THEN** the AST tree SHALL show `Program` as root, `ModeDecl` as first child, each `Rule[N]` with its label, and each attribute (`Distance`, `Alert`, `Priority`) as indented children with formatted values

#### Scenario: Print AST for a program with no mode
- **WHEN** compiling a program without `MODO` declaration with `--debug`
- **THEN** the AST tree SHALL show only `Program` root and `Rule` children, with no `ModeDecl` branch

#### Scenario: Print AST for a program with multiple rules
- **WHEN** compiling a program with 3 `DETECTAR` rules with `--debug`
- **THEN** the AST tree SHALL show `Rule[0]`, `Rule[1]`, `Rule[2]` as sequential children of `Program`, each with proper tree connectors

### Requirement: Debug output goes to stdout
All debug output from `--debug` mode SHALL be written to stdout. Parse errors and semantic errors SHALL continue to be written to stderr regardless of debug mode.

#### Scenario: Debug output and errors on different streams
- **WHEN** compiling a file with syntax errors using `--debug`
- **THEN** Phase 1 and Phase 2 debug output SHALL appear on stdout, and parse errors SHALL appear on stderr

### Requirement: CompilerDebugger encapsulates all printing logic
All debug printing SHALL be implemented in a `CompilerDebugger` module within `vt-cli` (`crates/vt-cli/src/debug.rs`). The `vt-lexer`, `vt-parser`, and `vt-core` crates SHALL NOT contain any debug-specific printing code. The pipeline orchestration in `main.rs` SHALL invoke `CompilerDebugger` methods with data produced by each phase.

#### Scenario: No debug code in core crates
- **WHEN** searching `vt-lexer/src/`, `vt-parser/src/`, `vt-core/src/`, and `vt-semantics/src/` for debug printing
- **THEN** no calls to `println!`, `eprintln!`, or `dbg!` related to `--debug` functionality SHALL exist in those crates
