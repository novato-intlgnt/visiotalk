---
status: not-started
phase: 1
updated: 2026-05-18
---

# Plan de Implementación: VisioTalk Compiler & Runtimes

## Goal

Construir un compilador end-to-end para el DSL VisioTalk en Rust, capaz de transpilar a Python (escritorio Linux) y JavaScript/React Native (móvil Android), con runtimes funcionales de visión computacional y TTS en ambas plataformas.

## Context & Decisions

| Decisión                             | Rationale                                                                                                    | Fuente             |
| ------------------------------------ | ------------------------------------------------------------------------------------------------------------ | ------------------ |
| Rust como lenguaje del core          | Seguridad de memoria, rendimiento nativo, interoperabilidad con Python (PyO3) y móviles (FFI/Native Modules) | `ref:questions.md` |
| lalrpop como parser generator        | Generador LR(1) declarativo nativo de Rust, integración tipada con el AST                                    | `ref:questions.md` |
| Python 3.11+ con uv para escritorio  | Balance de modernidad y compatibilidad; `uv` es el gestor de paquetes elegido                                | `ref:questions.md` |
| ONNX Runtime agnóstico en escritorio | Detecta automáticamente CUDA/OpenVINO en Linux sin cambiar código                                            | `ref:questions.md` |
| YOLOv8-nano como modelo base         | Estado del arte en velocidad/precisión, exportable a ONNX y TFLite                                           | `ref:questions.md` |
| React Native + TFLite para móvil     | Unifica desarrollo multiplataforma; TFLite es el estándar de Android con aceleración NNAPI/GPU               | `ref:questions.md` |
| Abstracción de profundidad en DSL    | Prepara el lenguaje para futuros sensores ToF/LiDAR sin breaking changes                                     | `ref:questions.md` |
| Monorepo                             | Facilita coherencia de versiones entre compiler core, runtime-desktop y runtime-mobile                       | `ref:questions.md` |

## Phase 1: Fundación y Scaffolding [COMPLETED]

- [x] 1.1 Crear estructura del monorepo (directorios `crates/`, `runtime-desktop/`, `runtime-mobile/`)
- [x] 1.2 Inicializar workspace Rust (`Cargo.toml` raíz con members)
- [x] 1.3 Configurar entorno Python 3.11+ en `runtime-desktop/` con `uv` (`pyproject.toml`, `.python-version`)
- [x] 1.4 Inicializar proyecto React Native en `runtime-mobile/` (última versión estable, TypeScript recomendado)
- [x] 1.5 Configurar herramientas de calidad: `rustfmt`, `clippy`, `ruff` (Python linter), ESLint/Prettier (RN)
- [x] **1.6 Crear documentación inicial de la gramática EBNF en `docs/grammar.md`** ← CURRENT

## Phase 2: Compiler Frontend - Lexer & Parser [PENDING]

- [ ] 2.1 Definir tipos de tokens en `crates/vt-lexer/`
- [ ] 2.2 Implementar lexer con `logos`: definir enum de tokens con `#[derive(Logos)]` y generar DFA automático
- [ ] 2.3 Escribir gramática EBNF formal en archivo `.lalrpop` dentro de `crates/vt-parser/`
- [ ] 2.4 Integrar `lalrpop` build script y generar parser LR(1)
- [ ] 2.5 Definir estructuras del AST en `crates/vt-core/`
- [ ] 2.6 Implementar action code en el parser para construir el AST
- [ ] 2.7 Escribir tests unitarios para lexer y parser (snapshot testing con `insta`)

## Phase 3: Compiler Middle-end - Análisis Semántico [PENDING]

- [ ] 3.1 Crear tabla de símbolos para identificadores y scopes
- [ ] 3.2 Validar existencia de etiquetas de modelo (vocabulario YOLO hardcodeado o cargado desde config)
- [ ] 3.3 Validar rangos de distancia (rechazar negativos, normalizar unidades)
- [ ] 3.4 Detectar colisiones de prioridad entre reglas del mismo nivel
- [ ] 3.5 Implementar sistema de errores semánticos con mensajes claros (fila/columna, sugerencias)
- [ ] 3.6 Tests de integración del pipeline léxico → sintáctico → semántico

## Phase 4: Compiler Backend - Desktop (Python) [PENDING]

- [ ] 4.1 Configurar motor de plantillas `askama` en `crates/vt-codegen/`
- [ ] 4.2 Diseñar plantilla base Python para el script de ejecución (`runtime-desktop/src/templates/main.py.j2`)
- [ ] 4.3 Implementar visitante del AST que mapea nodos a construcciones Python
- [ ] 4.4 Generar código de inicialización de cámara (OpenCV) en el script
- [ ] 4.5 Generar código de loop de inferencia ONNX Runtime
- [ ] 4.6 Generar código de evaluación de reglas y despacho de TTS
- [ ] 4.7 CLI del compilador (`vt-cli`) que acepta `.vt` de entrada y emite `.py`

## Phase 5: Runtime Desktop End-to-End [PENDING]

- [ ] 5.1 Implementar módulo `visio_engine.py` con pipeline de cámara en hilo separado
- [ ] 5.2 Integrar ONNX Runtime con carga de modelo YOLOv8 ONNX
- [ ] 5.3 Implementar estimación de distancia por tamaño de bounding box (normalizada por altura del frame)
- [ ] 5.4 Integrar motor TTS (`pyttsx3` con backend `espeak-ng`) con cola de mensajes no bloqueante
- [ ] 5.5 Script de descarga automática del modelo ONNX si no existe localmente
- [ ] 5.6 Test E2E: ejecutar script generado con una webcam y verificar alertas de voz

## Phase 6: Compiler Backend - Mobile (JavaScript/React Native) [PENDING]

- [ ] 6.1 Diseñar arquitectura orientada a eventos para el código JS generado
- [ ] 6.2 Crear plantilla JS base con hooks (`onObjectDetected`, `onDistanceThreshold`)
- [ ] 6.3 Implementar generador de código JS en `vt-codegen` (visitante alternativo al de Python)
- [ ] 6.4 Definir contrato de API entre JS generado y la app RN (qué funciones nativas se esperan)

## Phase 7: Mobile App & Compiler Integration [PENDING]

- [ ] 7.1 Compilar el core Rust (`crates/vt-*`) como biblioteca compartida para Android (`.so` para `arm64-v8a` y `x86_64`)
- [ ] 7.2 Crear TurboModule en React Native (New Architecture) para exponer funciones de compilación del core Rust vía JNI
- [ ] 7.3 Implementar pantalla de edición de `.vt` en React Native (TextInput con syntax highlighting básico)
- [ ] 7.4 Integrar botón "Compilar" que invoca el core nativo y genera el código JS en memoria
- [ ] 7.5 Guardar/cargar archivos `.vt` en el filesystem del dispositivo

## Phase 8: Mobile Runtime End-to-End [PENDING]

- [ ] 8.1 Integrar `react-native-vision-camera` para captura de frames
- [ ] 8.2 Cargar modelo TFLite (YOLOv8n) en hilo de background
- [ ] 8.3 Ejecutar inferencia TFLite y pasar resultados (bounding boxes) al código JS generado
- [ ] 8.4 Implementar evaluación de reglas en JS contra los objetos detectados
- [ ] 8.5 Integrar `react-native-tts` para síntesis de voz cuando se disparen alertas
- [ ] 8.6 Test E2E en dispositivo Android: detectar objeto y escuchar alerta

## Phase 9: Testing, CI/CD & Documentación [PENDING]

- [ ] 9.1 Configurar GitHub Actions: test Rust (`cargo test`), lint (`clippy`), format (`rustfmt`)
- [ ] 9.2 Configurar CI para Python (`ruff`, `mypy`, `pytest`)
- [ ] 9.3 Configurar CI para React Native (TypeScript check, lint, build Android)
- [ ] 9.4 Documentar API del compilador (`docs/api-reference.md`)
- [ ] 9.5 Tutorial de usuario: "Mi primer archivo .vt"

## Notes

- 2026-05-18: Fase 1 se enfoca en la gramática EBNF como fundamento de todo el parser.
- 2026-05-18: Se prioriza el runtime de escritorio (Fases 4-5) antes que el móvil para validar el DSL en un entorno controlado.
- 2026-05-18: El compilador Rust se compartirá entre escritorio Linux (via CLI) y Android (via biblioteca nativa `.so`) para garantizar coherencia semántica.
- 2026-05-18: Alineación de milestones: PRD Fase 1 ≈ IMP Phase 2, PRD Fase 2 ≈ IMP Phase 3+4, PRD Fase 3 ≈ IMP Phase 5, PRD Fase 4 ≈ IMP Phase 7, PRD Fase 5 ≈ IMP Phase 8.
