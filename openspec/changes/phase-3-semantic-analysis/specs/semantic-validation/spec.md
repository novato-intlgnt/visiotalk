## ADDED Requirements

### Requirement: Semantic analyzer validates empty program
The `vt_semantics::analyze` function SHALL return a semantic error when the parsed `Program` contains zero `DETECTAR` rules. The error SHALL have severity `Error` and include a message indicating that at least one rule is required.

#### Scenario: Empty rule list produces error
- **WHEN** `analyze()` is called on a `Program` with `rules: vec![]`
- **THEN** the function SHALL return `Err` containing a `SemanticError` with `severity: Error` and message "El programa debe contener al menos una regla DETECTAR"

#### Scenario: Single rule program passes
- **WHEN** `analyze()` is called on a `Program` with one valid `DetectRule`
- **THEN** the function SHALL return `Ok(())` (no errors or warnings)

### Requirement: Semantic analyzer validates distance range
The `vt_semantics::analyze` function SHALL validate that every `DistanceExpr` in the AST has a value strictly greater than 0. Values ≤ 0 SHALL produce an `Error`. Values ≥ 100 SHALL produce a `Warning` (exceeds typical sensor range).

#### Scenario: Zero distance produces error
- **WHEN** `analyze()` encounters a rule with `distancia: < 0.0m`
- **THEN** a `SemanticError` with `severity: Error` and message "La distancia debe ser mayor que 0" SHALL be returned

#### Scenario: Negative distance produces error
- **WHEN** `analyze()` encounters a rule with `distancia: > -5.0m`
- **THEN** a `SemanticError` with `severity: Error` and message "La distancia debe ser mayor que 0" SHALL be returned

#### Scenario: Distance >= 100 produces warning
- **WHEN** `analyze()` encounters a rule with `distancia: < 100.0m`
- **THEN** a `SemanticError` with `severity: Warning` and message "Distancia excede el rango típico del sensor" SHALL be returned

#### Scenario: Valid distance passes
- **WHEN** `analyze()` encounters a rule with `distancia: < 2.0m`
- **THEN** no semantic error for that attribute SHALL be produced

### Requirement: Semantic analyzer detects duplicate priorities
The `vt_semantics::analyze` function SHALL detect when two or more `DETECTAR` rules share the same `prioridad` value and emit a `Warning` identifying both rules by label.

#### Scenario: Two rules with same priority produce warning
- **WHEN** `analyze()` encounters rules for "persona" with priority 1 and "obstaculo" with priority 1
- **THEN** a `SemanticError` with `severity: Warning` and message containing both rule labels ("persona", "obstaculo") SHALL be returned

#### Scenario: All rules with unique priorities pass
- **WHEN** `analyze()` encounters rules with priorities 0, 1, and 2
- **THEN** no semantic error related to priority SHALL be produced

#### Scenario: Three-way priority collision produces one warning per pair
- **WHEN** `analyze()` encounters three rules all with priority 1
- **THEN** warnings SHALL be produced for each collision pair (labels A-B, A-C, B-C)

### Requirement: Semantic analyzer validates YOLO label vocabulary
The `vt_semantics::analyze` function SHALL validate that every `DetectRule.label` matches a known label in the YOLOv8-nano COCO vocabulary. Unknown labels SHALL produce a `Warning` with a suggestion listing valid labels.

#### Scenario: Unknown label produces warning
- **WHEN** `analyze()` encounters a rule with label "dragon" not in the YOLO vocabulary
- **THEN** a `SemanticError` with `severity: Warning` and message "La etiqueta 'dragon' no está en el vocabulario conocido del modelo" SHALL be returned

#### Scenario: Known label passes
- **WHEN** `analyze()` encounters a rule with label "persona" (mapped from COCO class "person")
- **THEN** no semantic error for that label SHALL be produced

#### Scenario: All 80 COCO classes are accepted
- **WHEN** `analyze()` validates a label for any of the 80 COCO class names
- **THEN** each SHALL be recognized as valid

### Requirement: Semantic analyzer validates alert text length
The `vt_semantics::analyze` function SHALL inspect every `alerta` attribute string. Empty alert strings and strings exceeding 200 characters SHALL produce `Warning`-level errors.

#### Scenario: Empty alert produces warning
- **WHEN** `analyze()` encounters a rule with `alerta: ""`
- **THEN** a `SemanticError` with `severity: Warning` and message "La alerta está vacía" SHALL be returned

#### Scenario: Overly long alert produces warning
- **WHEN** `analyze()` encounters a rule with `alerta:` containing 201+ characters
- **THEN** a `SemanticError` with `severity: Warning` and message "La alerta excede los 200 caracteres" SHALL be returned

#### Scenario: Valid-length alert passes
- **WHEN** `analyze()` encounters a rule with `alerta: "Obstáculo detectado"` (25 characters)
- **THEN** no semantic error for that attribute SHALL be produced

### Requirement: Semantic errors include file location metadata
Every `SemanticError` produced by `analyze()` SHALL include a `line` and `column` field indicating the source location in the `.vt` file. Errors on program-level checks (e.g., empty program) MAY use line 0, column 0.

#### Scenario: Rule-level error references source location
- **WHEN** `analyze()` produces an error for a distance on line 5
- **THEN** the `SemanticError` SHALL have `line: 5` and a meaningful `column` value

#### Scenario: Multiple errors preserve individual locations
- **WHEN** `analyze()` returns multiple errors for different rules
- **THEN** each error SHALL reference the line and column of its respective source element

### Requirement: Semantic analyzer reports all issues in one pass
The `analyze()` function SHALL collect ALL validation issues across ALL rules and return them in a single `Vec<SemanticError>` result. It SHALL NOT stop at the first error.

#### Scenario: Program with multiple rule violations
- **WHEN** `analyze()` processes a program where rule 1 has a negative distance and rule 2 has an unknown label
- **THEN** both errors SHALL be returned in the same `Err(Vec<SemanticError>)` result

#### Scenario: Warning-only result returns Ok
- **WHEN** `analyze()` encounters only Warning-level issues (no Errors)
- **THEN** the function SHALL return `Ok(())` — warnings are collected but do not block

### Requirement: CLI integrates semantic analysis into compilation pipeline
The `vt-cli` binary SHALL execute the full `lex → parse → analyze` pipeline and SHALL print all semantic errors and warnings to stderr when the analysis fails. The CLI SHALL exit with a non-zero status code when `analyze()` returns `Err`.

#### Scenario: Valid program compiles cleanly
- **WHEN** running `vt compile valid.vt` on a semantically correct file
- **THEN** the CLI SHALL print the AST and exit with status 0

#### Scenario: Semantic errors printed to stderr
- **WHEN** running `vt compile invalid.vt` where semantic analysis returns errors
- **THEN** the CLI SHALL print all semantic error messages to stderr and exit with a non-zero status code

#### Scenario: Semantic warnings printed but compilation succeeds
- **WHEN** running `vt compile warn.vt` where semantic analysis returns only warnings
- **THEN** the CLI SHALL print warnings to stderr, print the AST to stdout, and exit with status 0
