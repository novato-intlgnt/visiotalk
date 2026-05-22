## Context

El compilador de VisioTalk actualmente opera en modo "caja negra": `vt compile archivo.vt` imprime directamente el `Debug` del AST al stdout. Durante el desarrollo de fases posteriores (semántica, codegen), no hay visibilidad de los pasos intermedios — qué tokens generó el lexer, cómo se estructuró el árbol. Esto obliga a insertar `println!` manuales y recompilar.

La solución es un sistema de traza (`--debug`) que imprima cada fase del pipeline con formato visual legible, completamente modular (sin modificar lexer/parser/core) y con zero overhead cuando no está activo.

El pipeline actual en `main.rs` es lineal y hardcodeado:
```
leer fuente → parse(source) → println!("{:#?}", program)
```
Para Phase 3 se necesita insertar `analyze()` entre parse y la salida. Este diseño asegura que el `--debug` no genere conflicto con esa inserción futura.

## Goals / Non-Goals

**Goals:**
- Agregar flag `--debug` a `vt compile` que active la traza del pipeline completo
- Imprimir: (1) código fuente, (2) lista de tokens con tipo+valor, (3) AST jerárquico con caracteres de árbol
- Encapsular toda la lógica de impresión en `CompilerDebugger` (módulo en `vt-cli`)
- No modificar `vt-lexer`, `vt-parser`, `vt-core`, ni `vt-semantics`
- Estructurar el pipeline en `main.rs` como secuencia de stages extensible
- Sin `--debug`, el comportamiento es idéntico al actual

**Non-Goals:**
- Modificar la salida del modo normal (sin `--debug`)
- Agregar fases de debug para semántica o codegen (eso se hará cuando esas fases existan)
- Formato de salida configurable (JSON, YAML, etc.) — solo formato de árbol legible
- Piping o redirección del debug a archivo (solo stdout)
- Interactividad (step-through, breakpoints) — solo volcado completo

## Decisions

### D1: `CompilerDebugger` como módulo en `vt-cli`, no crate separado

**Elegido**: `crates/vt-cli/src/debug.rs` como módulo interno de `vt-cli`.

**Razón**: Es una herramienta de desarrollo sin caso de reuso fuera del CLI. Un crate `vt-debug` agregaría complejidad de workspace (nuevo `Cargo.toml`, dependencia, miembro) para ~200 líneas de código. Si en el futuro otros componentes necesitan debugging, se puede extraer — por ahora, YAGNI.

**Alternativa considerada**: Crate `vt-debug` separado. Rechazado — overhead innecesario para funcionalidad dev-only.

### D2: Pipeline como secuencia de stages extensible

**Elegido**: El pipeline se define como un `Vec` de stages `(nombre, función)` que se itera. Cada stage produce un resultado que el `CompilerDebugger` inspecciona si `--debug` está activo.

```
let stages: Vec<(&str, Box<dyn FnOnce() -> ...>)> = vec![
    ("FASE 1: CÓDIGO FUENTE", ...),
    ("FASE 2: TOKENS",         ...),
    ("FASE 3: AST",            ...),
];
```

**Razón**: Phase 3 (y futuras) solo necesitan agregar un elemento al vector. Sin conflicto de merge con el código de debug. Además, los stages son autocontenidos y testables individualmente.

**Alternativa considerada**: Código lineal con `if debug { ... }` entre cada fase. Rechazado — cada nueva fase requeriría duplicar el bloque `if debug`, y el orden de fases quedaría implícito.

### D3: Pretty-printer de AST con caracteres Unicode de árbol

**Elegido**: Implementar una función recursiva que recorra el AST y construya un `String` usando `├── `, `└── `, y `│   ` para la jerarquía. Cada nodo del AST tiene un formato visual definido (ver tabla abajo). La función usa un parámetro `prefix: &str` para trackear la indentación.

**Mapeo AST → display**:

| Nodo AST | Formato |
|----------|---------|
| `Program { mode, rules }` | `Program` + `ModeDecl` (si existe) + `Rule[0]...Rule[n]` |
| `ModeDecl { name }` | `ModeDecl: "nombre"` |
| `DetectRule { label, attributes }` | `Rule[N]: DETECTAR "label"` + atributos como hijos |
| `Attribute::Distance(expr)` | `Distance: <comparador> <valor> <unidad>` |
| `Attribute::Alert(s)` | `Alert: "texto"` |
| `Attribute::Priority(n)` | `Priority: <n>` |
| `DistanceExpr { comp, val, unit }` | Formateado inline: `< 2.0 m` |
| `Comparator` | `<`, `>`, `<=`, `>=`, `==` |
| `Unit` | `m`, `cm`, `ft`, `px` |

**Razón**: El `Debug` derive (`{:#?}`) produce output correcto pero ilegible para árboles anidados. Los caracteres Unicode de árbol son el estándar de facto para visualización de ASTs (usado por `rustc`, `clang`, `tree-sitter`). La función recursiva con prefix es el patrón canónico en Rust para pretty-printers de árboles.

**Alternativa considerada**: Usar `{:#?}` con `Debug`. Rechazado — no muestra jerarquía visual clara, mezcla nombres de variantes de enum con datos, y es difícil de auditar para estructuras anidadas.

### D4: Snapshot testing con insta para verificación

**Elegido**: Crear fixtures `.vt` en `crates/vt-cli/tests/fixtures/`, ejecutar `vt compile --debug fixture.vt` capturando stdout, y usar `insta::assert_snapshot!` para comparar.

**Razón**: El workspace ya tiene `insta` como dependencia y Phase 2 usó este patrón. Un solo snapshot cubre las tres fases simultáneamente. Los snapshots hacen que regresiones en el formato de salida sean inmediatamente visibles en code review.

**Alternativa considerada**: Tests unitarios por fase (assert sobre strings). Rechazado — frágil ante cambios de formato, más código de test que funcionalidad testeada.

### D5: Flujo de datos: el debugger recibe, no posee

**Elegido**: `CompilerDebugger` recibe referencias (`&str`, `&[(usize, Token, usize)]`, `&Program`). No clona ni almacena datos. `main.rs` es dueño de los datos y los pasa al debugger.

**Razón**: Separación clara de responsabilidades. El debugger es un renderer, no un collector. Si en el futuro se necesita debug asincrónico o streaming, los datos se pueden enviar por canal sin cambiar la firma.

## Risks / Trade-offs

- **[Risk] El pretty-printer puede divergir si se agregan nuevos nodos al AST**: Si Phase 3 o 4 agregan campos o variantes a `vt_core`, el pretty-printer debe actualizarse. → **Mitigation**: El pretty-printer usa `match` exhaustivo sobre los enums del AST — el compilador de Rust generará warning si falta una variante. Basta con correr `cargo clippy`.

- **[Risk] Archivos `.vt` grandes pueden generar salida de debug enorme**: Un programa con 100+ reglas DETECTAR produciría cientos de líneas de tokens y un AST profundo. → **Mitigation**: El debug es una herramienta de desarrollo; no se usa en producción. Si es necesario, se puede agregar `--debug-limit N` en el futuro.

- **[Trade-off] Los tokens se tokenizan dos veces en modo debug**: `vt-parser::parse()` internamente llama a `tokenize_with_locations()`. Para la Fase 2, el debugger necesitaría tokens antes de llamar al parser. → **Aceptado**: `tokenize_with_locations()` es O(n) y barato. La alternativa (modificar `parse()` para aceptar tokens pre-tokenizados) rompería la API pública y violaría el principio de no tocar `vt-parser`.

- **[Trade-off] Sin `--debug`, hay un `if` por cada fase**: Aunque el debugger no se instancia, el pipeline basado en stages tiene un `if debug { ... }` en el loop. → **Aceptado**: El branch es trivial y el compilador lo optimiza. No hay allocaciones ni trabajo extra en el camino `!debug`.

## Open Questions

- ¿Debería `--debug` también imprimir la fase de semántica (Phase 3) cuando se implemente? → Recomendación: sí, agregar el stage correspondiente en el PR de Phase 3.
- ¿Formato de timestamp o duración por fase? → Fuera de scope para esta iteración. Se puede agregar como `--debug --timing` en el futuro.
