# Temas de Aprendizaje por Fase - VisioTalk

**Última actualización:** 2026-05-18  
**Plataformas target:** Escritorio Linux + Android

Este documento lista los conocimientos técnicos que debes dominar (o al menos comprender a nivel funcional) para ejecutar cada fase del proyecto. Está organizado siguiendo el mismo orden del `IMPLEMENTATION_PLAN.md`.

---

## Phase 1: Fundación y Scaffolding

### Rust Toolchain y Ecosistema
- Instalación y gestión de toolchains con `rustup`
- Estructura de un workspace de Cargo (`Cargo.toml` raíz con `[workspace]`)
- Diferencia entre `bin`, `lib` y `dylib` crates
- Uso de `cargo new --lib` y organización de múltiples crates

### Gestión de Dependencias Python
- `uv` vs `pip` vs `poetry`: por qué `uv` es más rápido
- Estructura de `pyproject.toml` (`[project]`, `[project.optional-dependencies]`, `[build-system]`)
- Creación de entornos virtuales con `uv venv`

### React Native para Android
- Diferencia entre React Native CLI y Expo (se usará CLI para acceso nativo)
- Estructura de un proyecto RN con TypeScript
- Android Studio, SDK y NDK: qué versiones instalar
- Gradle y `android/app/build.gradle` (conceptos básicos)

### Herramientas de Calidad de Código
- `rustfmt` (formateo automático de Rust)
- `clippy` (linter oficial de Rust)
- `ruff` (linter/formateador ultra-rápido para Python)
- ESLint + Prettier para TypeScript/React Native

---

## Phase 2: Compiler Frontend - Lexer & Parser

### Teoría de Lenguajes Formales
- **Tokens:** qué son, tipos comunes (keywords, identifiers, literals, delimiters)
- **Autómatas Finitos Deterministas (DFA):** cómo un lexer reconoce patrones de caracteres
- **Gramáticas formales:** lenguajes regulares vs libres de contexto
- **Notación EBNF:** cómo expresar la sintaxis de VisioTalk formalmente
- **Análisis sintáctico:** top-down (descenso recursivo) vs bottom-up (LR, LALR)

### Herramientas Rust Específicas
- **logos:** cómo definir un enum de tokens con atributos `#[derive(Logos)]`
- **lalrpop:** sintaxis de archivos `.lalrpop`, reglas de producción, action code para construir el AST
- Build scripts en Cargo (`build.rs`) para generar código en tiempo de compilación

### Estructuras de Datos del Compilador
- Árbol de Sintaxis Abstracta (AST): por qué es agnóstico al backend
- Patrones de enums en Rust (`enum Expr { Literal(i64), Add(Box<Expr>, Box<Expr>) }`)
- Box, Rc, Arc: cuándo usar punteros en el AST

### Testing en Rust
- `cargo test` y organización de tests (`#[cfg(test)]`, `mod tests`)
- Snapshot testing con `insta`: cómo capturar y comparar salidas del parser/AST

---

## Phase 3: Análisis Semántico

### Conceptos de Análisis Estático
- Tabla de símbolos (Symbol Table): mapeo de identificadores a metadatos
- Scopes y anidamiento: cómo las reglas dentro de un `MODO` forman un scope
- Type checking básico: validar que `distancia` sea una expresión válida, que `prioridad` sea entero
- Análisis de flujo de datos: detectar colisiones entre reglas de la misma prioridad

### Patrones de Diseño
- **Visitor Pattern:** recorrer el AST sin modificarlo, aplicando diferentes validaciones
- **Builder Pattern:** construir mensajes de error con contexto progresivamente

### Reporte de Errores en Compiladores
- Diferencia entre error léxico, sintáctico y semántico
- **ariadne:** cómo generar reportes visuales con subrayado de línea/columna
- Diseño de mensajes de error útiles: qué información mostrar (ubicación, sugerencia, contexto)

---

## Phase 4: Compiler Backend - Desktop (Python)

### Generación de Código (Code Generation)
- Motores de plantillas: concepto de separar lógica y presentación
- **askama:** sintaxis de templates (`{% if %}`, `{{ variable }}`), integración con structs Rust
- Patrón Visitor aplicado a generación de código: un visitante por cada target (Python, JS)

### APIs del Runtime Python
- **OpenCV (`cv2`):** `VideoCapture`, lectura de frames, propiedades de cámara
- **ONNX Runtime:** `InferenceSession`, `run()`, manejo de inputs/outputs como tensores NumPy
- **pyttsx3:** inicialización del engine, `say()`, `runAndWait()`, configuración de voces en Linux
- **espeak-ng:** instalación y configuración como backend TTS en Linux

### Scripting y Automatización
- Diseñar un script Python auto-contenido que se genera desde templates
- Cómo inyectar reglas del DSL dentro de un loop de procesamiento

---

## Phase 5: Runtime Desktop End-to-End

### Concurrencia en Python
- `threading`: crear hilos, `Thread`, `Event`, `Queue`
- `queue.Queue`: patrón productor-consumidor (cámara produce frames, inferencia los consume)
- Diferencia entre CPU-bound e I/O-bound: por qué threading funciona para I/O (cámara, audio)
- GIL (Global Interpreter Lock): sus limitaciones y cómo evitar cuellos de botella

### Visión Computacional Práctica
- Formato de salida de YOLO: bounding boxes `(x1, y1, x2, y2)`, confianza, clase
- Non-Maximum Suppression (NMS): por qué es necesario y cómo funciona a alto nivel
- Estimación de distancia por tamaño aparente: relación inversa entre tamaño en píxeles y distancia
- Normalización de coordenadas: por qué usar proporciones en vez de píxeles absolutos

### Audio No Bloqueante
- Colas de mensajes TTS: acumular alertas y hablarlas secuencialmente sin detener la cámara
- Throttling de alertas: evitar spam de voz si un objeto permanece en el frame

---

## Phase 6: Compiler Backend - Mobile (JavaScript)

### Generación de Código JavaScript
- Arquitectura orientada a eventos: callbacks (`onObjectDetected`, `onAlertTriggered`)
- Cómo generar funciones JS desde Rust usando templates (`askama` puede generar cualquier texto)
- Módulos ES6 vs CommonJS: qué formato usar para el código generado

### Integración React Native - Nativo
- **JSI (JavaScript Interface):** cómo RN comunica JS con código nativo (C++/Rust)
- **TurboModules:** el sistema moderno de RN para exponer APIs nativas a JS
- **New Architecture (Fabric + TurboModules):** por qué es necesario para rendimiento nativo
- Contratos de API: definir qué funciones JS espera del lado nativo (ej. `NativeModules.VTCompiler.compile()`)

---

## Phase 7: Mobile App & Compiler Integration

### Compilación Cruzada (Cross-compilation)
- **cargo-ndk:** qué hace, cómo instalarlo, arquitecturas Android (`arm64-v8a`, `x86_64`)
- NDK (Native Development Kit): por qué es necesario para compilar Rust a Android
- Estructura de `jniLibs/` en un proyecto Android

### JNI (Java Native Interface) - Conceptos Básicos
- Cómo Java/Kotlin carga bibliotecas `.so` con `System.loadLibrary()`
- Declaración de métodos `native` en Java y su correspondencia en Rust (vía FFI)
- Manejo de strings entre Java y Rust (`JString`, conversión UTF-8)

### React Native: Módulos Nativos
- Crear un TurboModule en TypeScript que declare los métodos expuestos
- Implementar el módulo nativo en Java/Kotlin que invoca el `.so` de Rust
- Registrar el módulo en `MainApplication.java` / `MainApplication.kt`

### FileSystem y Storage en Android
- `react-native-fs`: leer/escribir archivos en almacenamiento interno del app
- AsyncStorage: guardar preferencias y metadatos del archivo `.vt`
- Permisos de Android: `READ_EXTERNAL_STORAGE`, `WRITE_EXTERNAL_STORAGE` (conceptos modernos con Scoped Storage)

---

## Phase 8: Mobile Runtime End-to-End

### Cámara en React Native
- `react-native-vision-camera`: conceptos de `Camera`, `useCameraDevice`, `useFrameProcessor`
- Frame Processors: ejecutar código en cada frame de la cámara (Worklets)
- Diferencia entre JS thread y UI thread vs Camera thread

### TensorFlow Lite en Android
- Formato `.tflite`: modelo cuantizado vs FP16
- Interprete de TFLite: inicialización, asignación de tensores, `run()`
- Delegados (Delegates): NNAPI, GPU, CPU — cómo elegir el acelerador
- Post-procesamiento de detecciones en JS o nativo

### Evaluación de Reglas en Tiempo Real
- Cómo el código JS generado recibe objetos detectados y evalúa condiciones
- Latencia: objetivo < 500ms entre detección y alerta de voz
- Optimizaciones: no evaluar todas las reglas en cada frame si no es necesario

### Text-to-Speech en Android
- `react-native-tts`: inicialización, `speak()`, configuración de idioma y velocidad
- Gestión de cola de habla: priorizar alertas de mayor urgencia

---

## Phase 9: Testing, CI/CD y Documentación

### Testing Integrado
- `cargo test`: tests unitarios en Rust, tests de integración en `tests/`
- `pytest`: estructura de tests en Python, fixtures, mocks
- Testing en React Native: Jest para lógica JS, E2E con Detox o Maestro (opcional)

### GitHub Actions
- Conceptos: workflows, jobs, steps, runners, `ubuntu-latest`
- Matrices de build: compilar para múltiples arquitecturas
- Caching de dependencias: `cargo`, `npm`, `uv`
- Artefactos: guardar APKs o scripts generados como artefactos descargables

### Documentación Técnica
- Escribir especificaciones en Markdown claro y estructurado
- Documentar APIs con ejemplos de entrada/salida
- Diagramas de arquitectura (pueden ser en ASCII o herramientas como Mermaid)

---

## Ruta de Aprendizaje Recomendada

Si estás empezando desde cero, esta es una secuencia pragmática:

1. **Rust básico** (variables, enums, pattern matching, ownership) → necesario para Phase 1-4
2. **Teoría de compiladores** (tokens, AST, parsing) → necesario para Phase 2-3
3. **Python + OpenCV + ONNX** → necesario para Phase 4-5
4. **React Native básico** (components, hooks, navigation) → necesario para Phase 6-8
5. **JNI / TurboModules** → necesario para Phase 7
6. **GitHub Actions** → necesario para Phase 9

No necesitas ser experto en todo antes de empezar. Cada fase del proyecto te obligará a aprender lo justo y necesario para avanzar.

---

*Fin del documento de Temas de Aprendizaje*
