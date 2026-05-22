## Why

El compilador de VisioTalk es una caja negra durante el desarrollo: cuando un `.vt` produce resultados inesperados, no hay forma de inspeccionar qué tokens generó el lexer o cómo se estructuró el AST sin insertar `println!` manuales. Esto hace que el debugging de fases posteriores (Phase 3 semantic analysis, Phase 4 codegen) sea innecesariamente lento. Antes de implementar Phase 3, necesitamos visibilidad completa del pipeline.

## What Changes

- **Flag `--debug`** en `vt compile`: activa el modo de traza. Sin el flag, el compilador opera exactamente igual que ahora (zero overhead).
- **Fase 1 — Código fuente**: imprime el contenido del `.vt` entre separadores visuales.
- **Fase 2 — Tokens**: imprime la lista secuencial de tokens (tipo + valor léxico, uno por línea con índice).
- **Fase 3 — AST**: imprime el árbol sintáctico con caracteres de árbol (`├──`, `└──`, `│`) en lugar del volcado `Debug` plano, mostrando jerarquía de nodos.
- **`CompilerDebugger`**: módulo nuevo en `vt-cli` (`crates/vt-cli/src/debug.rs`) que encapsula toda la lógica de impresión. Lexer, parser y core no se modifican.
- **Pipeline extensible**: la orquestación en `main.rs` se estructura como secuencia de stages para que Phase 3 (y futuras) se inserten sin conflicto.

## Capabilities

### New Capabilities
- `compiler-debug`: Traza paso a paso del pipeline del compilador activada con `--debug`. Imprime código fuente, tokens y AST jerárquico con formato de árbol.

### Modified Capabilities
- `compiler-pipeline`: El requirement "CLI executes the lex → parse pipeline" se modifica para que cuando `--debug` esté activo, el CLI imprima salida estructurada de cada fase en lugar del volcado `Debug` plano actual.

## Impact

- **Affected crates**: `vt-cli` (nuevo módulo `debug.rs`, cambios en `main.rs`). `vt-lexer`, `vt-parser`, `vt-core`, `vt-semantics` sin cambios.
- **No new dependencies**: `clap` ya está en el workspace. Caracteres de árbol son Unicode literal.
- **No breaking changes**: sin `--debug`, el comportamiento es idéntico al actual.
- **Testing**: nuevos snapshots de `insta` en `vt-cli/tests/` con fixtures `.vt`.
