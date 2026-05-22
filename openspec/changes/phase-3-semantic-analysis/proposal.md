## Why

The lexer and parser (Phase 2) validate that `.vt` syntax is well-formed, but they cannot catch logical errors that would cause runtime failures or incorrect audio alerts. A program like `DETECTAR persona { distancia: < -5.0m }` parses successfully but describes a physically impossible negative distance. Similarly, two rules at the same priority level create ambiguous alert dispatch, and object labels not recognized by YOLOv8 produce silent failures. Phase 3 adds a semantic analysis pass that validates the parsed AST against domain constraints — rejecting invalid configurations before they reach code generation or runtime execution. This is the last compiler phase that can catch user mistakes early, making it a critical safety layer for a system designed to assist visually impaired users.

## What Changes

- Implement a **symbol table** for tracking identifiers and scopes within the AST
- Add **YOLO label validation**: check that `DETECTAR <label>` uses labels recognized by the YOLO model vocabulary
- Add **distance range validation**: reject negative/zero distances, warn on values exceeding 100 (sensor range limit)
- Add **priority collision detection**: emit warnings when two or more rules share the same priority level
- Implement a **structured semantic error system** with file location (line/column) and actionable suggestions (`ariadne`-style reporting)
- Write **integration tests** for the full `lex → parse → analyze` pipeline with snapshot testing

## Capabilities

### New Capabilities
- `semantic-validation`: Validates parsed ASTs against VisioTalk domain rules — label vocabulary, distance ranges, priority uniqueness, empty programs, and alert length. Produces structured errors with line/column information and actionable suggestions.

### Modified Capabilities
<!-- No existing capabilities have their requirements changed. This is purely additive. -->

## Impact

- **Affected crates**: `vt-semantics` (new implementation), `vt-core` (may need label vocabulary types), `vt-cli` (integrate semantic analysis into pipeline)
- **No API changes**: `vt-core::Program`, `vt-core::DetectRule`, and parser output types are already designed to be consumed by semantic analysis
- **New dependency**: `ariadne` for structured error diagnostics (already planned in tech stack)
- **Docs**: `docs/grammar.md` section "Restricciones del Analizador Semántico" defines the validation table this phase implements
