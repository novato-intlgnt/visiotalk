# Plan: Cross-Document Consistency Fixes

## TL;DR
> **Quick Summary**: Aplicar 11 correcciones puntuales en PRD.md, IMPLEMENTATION_PLAN.md y TECH_STACK.md para alinearlos con las decisiones confirmadas: Linux exclusivo (escritorio), Android exclusivo (móvil), `logos` para lexer, `espeak-ng` como backend TTS.
> 
> **Deliverables**:
> - `docs/PRD.md` con 6 correcciones aplicadas
> - `docs/IMPLEMENTATION_PLAN.md` con 2 correcciones aplicadas
> - `docs/TECH_STACK.md` con 3 correcciones aplicadas
> 
> **Estimated Effort**: Quick
> **Parallel Execution**: YES - 3 waves (un archivo por wave, totalmente independientes)

---

## Context

### Original Request
El usuario pidió analizar PRD.md, IMPLEMENTATION_PLAN.md y TECH_STACK.md para detectar y corregir inconsistencias tras las últimas decisiones: Linux exclusivo, Android exclusivo, `logos` como lexer, `espeak-ng` como backend TTS en Linux.

### Research Findings
Análisis cruzado de los tres documentos reveló 11 inconsistencias (ver sección de TODOs abajo). Ninguna es arquitectónica — todas son referencias obsoletas a Windows, iOS, DirectML, o `gTTS`.

---

## Work Objectives

### Core Objective
Alinear los tres documentos de referencia del proyecto con el stack tecnológico confirmado.

### Concrete Deliverables
- `docs/PRD.md` — Actualizado: sin Windows, sin iOS, sin DirectML, TTS corregido, `logos` agregado.
- `docs/IMPLEMENTATION_PLAN.md` — Actualizado: `.so` para Android, lexer con `logos`.
- `docs/TECH_STACK.md` — Actualizado: sin Windows, backend TTS especificado.

### Definition of Done
- [ ] Las 11 correcciones aplicadas en los 3 archivos
- [ ] Ningún documento contiene referencias a Windows, iOS, DirectML o `gTTS`

### Must Have
- Todas las correcciones listadas abajo

### Must NOT Have (Guardrails)
- NO cambios en estructura de documentos ni en fases del plan
- NO cambios en gramática EBNF ni en el DSL
- NO añadir nuevo contenido más allá de las correcciones necesarias

---

## Verification Strategy

### Test Decision
- **Automated tests**: None
- **Agent-Executed QA**: Sí — verificar con grep que no queden referencias a Windows/iOS/DirectML/gTTS en los archivos modificados

---

## TODOs

- [ ] 1. Fix PRD.md - Windows/Linux → Linux + iOS → Android + TTS

  **What to do**:
  - Línea 21: Cambiar `Linux/Windows` por `Linux` (y agregar `Android` en lugar de "dispositivos móviles")
  - Línea 56: Cambiar `bibliotecas nativas (Android/iOS)` por `bibliotecas nativas (Android)`
  - Línea 102: Cambiar `CUDA/OpenVINO/DirectML` por `CUDA/OpenVINO`
  - Línea 104: Cambiar `pyttsx3 o gTTS offline` por `pyttsx3 con backend espeak-ng en Linux`
  - Línea 209: Eliminar la línea `├── ios/` del árbol del monorepo
  - Línea 237: Cambiar `TalkBack (Android) y VoiceOver (iOS)` por `TalkBack (Android)`
  - Línea 93 (tabla 4.1): Agregar fila para `logos` (Lexer) en la tabla del Compilador Core

  **Must NOT do**:
  - No modificar la gramática EBNF
  - No cambiar la estructura de fases/milestones

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: None needed — son ediciones de strings simples

  **Parallelization**: Wave 1 (con Tasks 2 y 3)

  **References**:
  - `docs/PRD.md` — Archivo a modificar
  - `docs/IMPLEMENTATION_PLAN.md` — Para verificar consistencia de target platform
  - `docs/TECH_STACK.md` — Para verificar stack correcto

  **Acceptance Criteria**:
  - [ ] READ docs/PRD.md → no contiene "Windows", "iOS", "DirectML", "gTTS"
  - [ ] READ docs/PRD.md → contiene "Linux", "Android", "espeak-ng", "logos"

  **QA Scenarios**:
  ```
  Scenario: Verify no obsolete platform references in PRD
    Tool: Bash (grep)
    Steps:
      1. grep -i "windows\|directml\|gtTs" docs/PRD.md
      2. Assert: zero matches
    Expected Result: No output (no matches)
    Evidence: .sisyphus/evidence/task-1-prd-clean.md

  Scenario: Verify Android-only mobile references
    Tool: Bash (grep)
    Steps:
      1. grep -i "ios" docs/PRD.md
      2. Assert: zero matches
    Expected Result: No output (no matches)
    Evidence: .sisyphus/evidence/task-1-prd-no-ios.md
  ```

  **Commit**: YES
  - Message: `docs(prd): fix platform refs - Linux/Android only, logos lexer, espeak-ng TTS`
  - Files: `docs/PRD.md`

- [ ] 2. Fix IMPLEMENTATION_PLAN.md - Android .so + logos lexer

  **What to do**:
  - Línea 82: Cambiar `biblioteca estática para Android (.a) e iOS (.a/.xcframework)` por `biblioteca compartida para Android (.so arm64/x86_64)`
  - Líneas 37-38: Cambiar `Implementar lexer/scanner manual en Rust (conversión de caracteres a tokens)` por `Implementar lexer con logos (crate declarativo de autómatas finitos)`

  **Must NOT do**:
  - No modificar la estructura de fases
  - No alterar el frontmatter YAML

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: None needed

  **Parallelization**: Wave 1 (con Tasks 1 y 3)

  **References**:
  - `docs/IMPLEMENTATION_PLAN.md` — Archivo a modificar
  - `docs/TECH_STACK.md` — `logos` specs en tabla de Compilador Core

  **Acceptance Criteria**:
  - [ ] READ docs/IMPLEMENTATION_PLAN.md → no contiene "iOS", "manual", "scanner manual"
  - [ ] READ docs/IMPLEMENTATION_PLAN.md L82 → contiene ".so"
  - [ ] READ docs/IMPLEMENTATION_PLAN.md L37-38 → contiene "logos"

  **QA Scenarios**:
  ```
  Scenario: Verify iOS references removed from plan
    Tool: Bash (grep)
    Steps:
      1. grep -i "ios\|xcframework" docs/IMPLEMENTATION_PLAN.md
      2. Assert: zero matches
    Expected Result: No output
    Evidence: .sisyphus/evidence/task-2-plan-no-ios.md

  Scenario: Verify logos replaces manual lexer
    Tool: Bash (grep)
    Steps:
      1. grep "logos" docs/IMPLEMENTATION_PLAN.md
      2. Assert: at least 1 match (task 2.2 updated)
    Expected Result: line containing "logos"
    Evidence: .sisyphus/evidence/task-2-plan-logos.md
  ```

  **Commit**: YES
  - Message: `docs(plan): fix Android .so target, use logos for lexer`
  - Files: `docs/IMPLEMENTATION_PLAN.md`

- [ ] 3. Fix TECH_STACK.md - Remove Windows, add espeak-ng

  **What to do**:
  - Línea 4: Cambiar `Escritorio (Linux/Windows)` por `Escritorio (Linux)`
  - Líneas 195-196: Eliminar la fila de la tabla que dice `Escritorio (Windows) | x86_64 | ONNX Runtime con DirectML...`
  - Línea 67: Cambiar `TTS offline multiplataforma` por `TTS offline en Linux (backend espeak-ng)`

  **Must NOT do**:
  - No modificar versiones de dependencias
  - No alterar los snippets de código (Cargo.toml, pyproject.toml, package.json)

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: None needed

  **Parallelization**: Wave 1 (con Tasks 1 y 2)

  **References**:
  - `docs/TECH_STACK.md` — Archivo a modificar
  - `docs/PRD.md` — Para verificar consistencia de stack

  **Acceptance Criteria**:
  - [ ] READ docs/TECH_STACK.md → no contiene "Windows", "DirectML"
  - [ ] READ docs/TECH_STACK.md → contiene "espeak-ng"
  - [ ] READ docs/TECH_STACK.md L4 → contiene solo "Linux"

  **QA Scenarios**:
  ```
  Scenario: Verify Windows references removed from tech stack
    Tool: Bash (grep)
    Steps:
      1. grep -i "windows\|directml" docs/TECH_STACK.md
      2. Assert: zero matches
    Expected Result: No output
    Evidence: .sisyphus/evidence/task-3-stack-no-windows.md

  Scenario: Verify espeak-ng specified as TTS backend
    Tool: Bash (grep)
    Steps:
      1. grep "espeak-ng" docs/TECH_STACK.md
      2. Assert: at least 1 match
    Expected Result: line containing "espeak-ng"
    Evidence: .sisyphus/evidence/task-3-stack-espeak.md
  ```

  **Commit**: YES
  - Message: `docs(stack): Linux-only desktop, espeak-ng TTS backend`
  - Files: `docs/TECH_STACK.md`

---

## Final Verification Wave

- [ ] F1. **Plan Compliance Audit** — `oracle`
  Verificar que los 3 archivos fueron modificados exactamente con los 11 cambios especificados.
  Output: `Changes [11/11] | PRD [6/6] | IMPL [2/2] | TECH [3/3] | VERDICT: APPROVE/REJECT`

- [ ] F2. **Cross-Document Consistency** — `unspecified-high`
  Leer los 3 archivos y verificar: no hay "Windows", "iOS", "DirectML", "gTTS" en ninguno. Todas las secciones de stack coinciden entre documentos.
  Output: `Grep: [CLEAN/N issues] | Consistency: [PASS/FAIL] | VERDICT`

- [ ] F3. **Real Manual QA** — `unspecified-high`
  Ejecutar los escenarios QA de cada tarea. Verificar que los archivos `PRD.md`, `IMPLEMENTATION_PLAN.md`, `TECH_STACK.md` son consistentes entre sí y con las decisiones de `docs/questions.md`.
  Output: `Scenarios [6/6 pass] | VERDICT`

- [ ] F4. **Scope Fidelity Check** — `deep`
  Verificar que solo se hicieron los 11 cambios especificados, sin creep de scope.
  Output: `Tasks [3/3 compliant] | Contamination [CLEAN/N issues] | VERDICT`

---

## Commit Strategy

- **1**: `docs(prd): fix platform refs - Linux/Android only, logos lexer, espeak-ng TTS` — `docs/PRD.md`
- **2**: `docs(plan): fix Android .so target, use logos for lexer` — `docs/IMPLEMENTATION_PLAN.md`
- **3**: `docs(stack): Linux-only desktop, espeak-ng TTS backend` — `docs/TECH_STACK.md`

---

## Success Criteria

### Verification Commands
```bash
grep -rn "Windows\|DirectML\|iOS\|gTTS" docs/PRD.md docs/IMPLEMENTATION_PLAN.md docs/TECH_STACK.md
# Expected: zero matches

grep -rn "espeak-ng\|logos" docs/PRD.md docs/IMPLEMENTATION_PLAN.md docs/TECH_STACK.md
# Expected: matches in all three files
```

### Final Checklist
- [ ] PRD.md: 6/6 correcciones aplicadas, solo Linux + Android
- [ ] IMPLEMENTATION_PLAN.md: 2/2 correcciones aplicadas, `.so` + `logos`
- [ ] TECH_STACK.md: 3/3 correcciones aplicadas, sin Windows + espeak-ng
