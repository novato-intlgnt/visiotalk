## MODIFIED Requirements

### Requirement: CLI executes the lex → parse pipeline
The `vt` binary SHALL invoke `vt_lexer::tokenize` on the input source, pass the resulting token stream to `vt_parser::parse`, and output the result to stdout. When the `--debug` flag is active, the output SHALL include structured trace information for each pipeline phase (source code, token list, AST tree). When `--debug` is not active, the CLI SHALL print the Debug representation of the resulting `vt_core::Program`.

#### Scenario: Successful pipeline with debug
- **WHEN** running `vt compile --debug` on a valid `.vt` file
- **THEN** the CLI SHALL print Phase 1 (source), Phase 2 (tokens), and Phase 3 (AST tree) to stdout and exit with status 0

#### Scenario: Successful pipeline without debug
- **WHEN** running `vt compile` on a valid `.vt` file without `--debug`
- **THEN** the CLI SHALL print the Debug representation of the resulting `vt_core::Program` and exit with status 0

#### Scenario: Pipeline stops on parse error
- **WHEN** running `vt compile` on a `.vt` file with syntax errors
- **THEN** the CLI SHALL print the parse errors (including line/column and expected tokens) to stderr and exit with a non-zero status code
