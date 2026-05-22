## ADDED Requirements

### Requirement: CLI accepts .vt file and target platform
The `vt` binary SHALL accept an input file path and an optional target platform argument via command-line flags. It SHALL validate that the input file exists and has a `.vt` extension.

#### Scenario: Compile with default target
- **WHEN** running `vt compile input.vt` without specifying a target
- **THEN** the CLI SHALL default to `python` target and proceed with compilation

#### Scenario: Compile with explicit target
- **WHEN** running `vt compile input.vt --target python` or `vt compile input.vt --target javascript`
- **THEN** the CLI SHALL use the specified target platform

#### Scenario: Reject missing input file
- **WHEN** running `vt compile nonexistent.vt`
- **THEN** the CLI SHALL print an error message and exit with a non-zero status code

### Requirement: CLI executes the lex → parse pipeline
The `vt` binary SHALL invoke `vt_lexer::tokenize` on the input source, pass the resulting token stream to `vt_parser::parse`, and print the resulting AST in Debug format to stdout.

#### Scenario: Successful pipeline execution
- **WHEN** running `vt compile` on a valid `.vt` file
- **THEN** the CLI SHALL print the Debug representation of the resulting `vt_core::Program` and exit with status 0

#### Scenario: Pipeline stops on parse error
- **WHEN** running `vt compile` on a `.vt` file with syntax errors
- **THEN** the CLI SHALL print the parse errors (including line/column and expected tokens) and exit with a non-zero status code

### Requirement: Lexer tokenizes all valid tokens correctly
The `vt-lexer` crate SHALL tokenize a `.vt` source string into a `Vec<Token>` that preserves token order. Every keyword, delimiter, comparator, unit, number literal, string literal, and identifier SHALL be recognized as its correct `Token` variant.

#### Scenario: Tokenize all keywords
- **WHEN** tokenizing the string `"MODO DETECTAR"` 
- **THEN** the output SHALL be `[Token::KwModo, Token::KwDetectar]`

#### Scenario: Tokenize all comparators
- **WHEN** tokenizing `"< > <= >= =="`
- **THEN** the output SHALL be `[Token::Lt, Token::Gt, Token::Le, Token::Ge, Token::Eq]`

#### Scenario: Tokenize all units
- **WHEN** tokenizing `"m cm ft px"`
- **THEN** the output SHALL be `[Token::UnitMeters, Token::UnitCentimeters, Token::UnitFeet, Token::UnitPixels]`

#### Scenario: Tokenize string and number literals
- **WHEN** tokenizing `"\"Alerta\" 2.5 42"`
- **THEN** the output SHALL contain `Token::StringLit("Alerta")`, `Token::NumberLit(2.5)`, and `Token::NumberLit(42.0)`

#### Scenario: Skip whitespace and comments
- **WHEN** tokenizing a string containing spaces, tabs, newlines, and `//` comments
- **THEN** those characters SHALL not appear in the output token stream

#### Scenario: Handle empty input
- **WHEN** tokenizing an empty string
- **THEN** the output SHALL be an empty `Vec<Token>`
