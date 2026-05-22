# VisioTalk - Product Requirements Document (PRD)

**Versión:** 1.0
**Fecha:** 2026-05-18
**Estado:** Borrador para revisión

---

## 1. Resumen Ejecutivo

VisioTalk es un ecosistema de software centrado en un Lenguaje de Dominio Específico (DSL) diseñado para la asistencia y navegación independiente de personas con discapacidad visual. El sistema consta de un compilador/transpilador que traduce archivos de configuración declarativa `.vt` a implementaciones optimizadas de bajo nivel, integrando flujos de vídeo en tiempo real, inferencia con modelos de IA (Visión Computacional) y síntesis de voz (TTS) para generar alertas auditivas espaciales.

---

## 2. Objetivos y Alcance

### 2.1 Objetivos Principales

- **Abstracción de Complejidad:** Separar la lógica de asistencia (el "qué") de la fontanería técnica de IA (el "cómo").
- **Seguridad por Diseño:** Utilizar análisis estático (léxico, sintáctico, semántico) para validar reglas de navegación antes de la ejecución.
- **Portabilidad Dual:** Compilar a un script standalone optimizado para escritorio (Linux) o generar un artefacto intermedio para ejecución nativa en dispositivos Android.

### 2.2 Alcance del MVP (Fase 1-3)

- Gramática del DSL VisioTalk.
- Compilador core en Rust con frontend (lexer/parser) y análisis semántico.
- Backend de transpilación a Python para escritorio.
- Runtime de escritorio funcional (video + ONNX + YOLO + TTS).
- Estructura base del monorepo y CI/CD.

### 2.3 Alcance Extendido (Fase 4+)

- Backend de transpilación a JavaScript/React Native.
- Aplicativo Android con editor `.vt` y mini-compilador embebido.
- Runtime Android con TFLite.
- Soporte para sensores de profundidad.
- *Futuro:* Soporte iOS (VoiceOver, CoreML).

---

## 3. Arquitectura del Sistema

### 3.1 Visión General

El pipeline de compilación sigue las fases clásicas:

1. **Análisis Léxico:** Tokenización del archivo `.vt`.
2. **Análisis Sintáctico:** Construcción del AST según gramática EBNF.
3. **Análisis Semántico:** Validación de etiquetas, rangos y prioridades.
4. **Generación de Código:**
   - Escritorio: Plantillas Python (Jinja2/Askama).
   - Móvil: Módulos JavaScript orientados a eventos.

### 3.2 Lenguaje del Compilador Core

**Decisión:** Rust (primario).
**Rationale:** Seguridad de memoria sin garbage collector, rendimiento comparable a C++, ecosistema moderno de crates para parsing (`lalrpop`, `pest`), y capacidad de compilación a bibliotecas nativas (Android/iOS) y WebAssembly. C++ se reserva para futuros módulos nativos de bajo nivel si el perfilado lo indica.

### 3.3 Parser Generator

**Decisión:** Generador automático (`lalrpop`).
**Rationale:** Declarativo, integrado con el ecosistema Rust, genera parsers LR(1) eficientes y tipados, permitiendo iteración rápida sobre la gramática.

### 3.4 AST y Representación Intermedia

El AST es agnóstico al backend. Cada nodo es un enum/struct en Rust que captura:

- Declaraciones de modo (`MODO`).
- Reglas de detección (`DETECTAR`) con atributos.
- Expresiones de distancia (con abstracción para futuros sensores de profundidad).
- Configuraciones de alerta y prioridad.

### 3.5 Análisis Semántico

Validaciones críticas de seguridad:

- **Etiquetas de modelo:** Verificar que los objetos a detectar pertenezcan al vocabulario del modelo YOLO subyacente.
- **Rangos lógicos:** Distancias negativas o fuera del rango físico del sensor deben ser rechazadas.
- **Colisión de prioridades:** Prevenir contradicciones entre reglas de la misma prioridad que compitan por el mismo recurso de audio.
- **Tipado estricto:** Validar que `alerta` sea string, `prioridad` sea entero positivo, `distancia` sea una expresión dimensional válida.

---

## 4. Stack Tecnológico

### 4.1 Compilador Core

| Componente | Tecnología                                | Justificación                                 |
| ---------- | ----------------------------------------- | --------------------------------------------- |
| Lenguaje   | Rust                                      | Seguridad, rendimiento, interoperabilidad     |
| Parser Gen | `lalrpop`                                 | Generador LR(1) nativo de Rust                |
| Templates  | `askama`                                  | Motor de plantillas tipo Jinja en Rust        |
| Testing    | `cargo test` + `insta` (snapshot testing) | Verificación del AST y salidas                |
| FFI        | `PyO3` (opcional)                         | Exponer el compilador a Python si se requiere |

### 4.2 Runtime Escritorio

| Componente      | Tecnología                 | Justificación                                     |
| --------------- | -------------------------- | ------------------------------------------------- |
| Lenguaje        | Python 3.11+               | Balance modernidad/compatibilidad                 |
| Gestor Paquetes | `uv`                       | Rápido, moderno, reemplazo a pip/poetry           |
| Captura Video   | OpenCV + threading         | Estándar de facto, manejo de cámaras              |
| Inferencia      | ONNX Runtime (Python)      | Agnóstico a hardware en Linux (CUDA/OpenVINO)     |
| Modelo IA       | YOLOv8-nano (Ultralytics)  | Exportable a ONNX, equilibrio velocidad/precisión |
| TTS             | `pyttsx3` (+ `espeak-ng`)  | Síntesis de voz no bloqueante (backend Linux)     |
| Concurrencia    | `asyncio` / `threading`    | Pipeline de cámara e inferencia asíncrono         |

### 4.3 Runtime Móvil

| Componente    | Tecnología                | Justificación                             |
| ------------- | ------------------------- | ----------------------------------------- |
| Framework     | React Native              | Desarrollo unificado, ecosistema maduro   |
| Inferencia    | TensorFlow Lite           | Estándar Android, aceleración GPU/NNAPI   |
| Módulo Nativo | Rust via JSI/TurboModules | Reutilizar core del compilador en la app  |
| Editor        | React Native TextInput    | Edición nativa de archivos `.vt`          |
| TTS           | React Native TTS          | Acceso unificado a motores nativos de voz |

---

## 5. Especificación del DSL VisioTalk

### 5.1 Ejemplo Completo

```vt
MODO navegacion_urbana

DETECTAR persona {
    distancia: < 2.0m
    alerta: "Persona cercana a tu derecha"
    prioridad: 2
}

DETECTAR vehiculo {
    distancia: < 5.0m
    alerta: "Vehículo detectado"
    prioridad: 1
}

DETECTAR obstaculo_bajo {
    distancia: < 1.0m
    alerta: "Obstáculo en el suelo"
    prioridad: 3
}
```

### 5.2 Gramática EBNF (High-Level)

```ebnf
program       ::= modo_decl? , rule_decl+ ;
modo_decl     ::= "MODO" , identifier ;
rule_decl     ::= "DETECTAR" , identifier , "{" , attribute* , "}" ;
attribute     ::= key , ":" , value ;
key           ::= "distancia" | "alerta" | "prioridad" ;
value         ::= distance_expr | string_literal | integer_literal ;
distance_expr ::= comparator , numeric_literal , unit ;
comparator    ::= "<" | ">" | "<=" | ">=" | "==" ;
unit          ::= "m" | "cm" | "ft" | "px" ;  (* px para tamaño relativo de bounding box *)
identifier    ::= letter , { letter | digit | "_" } ;
string_literal::= '"' , { character } , '"' ;
numeric_literal::= digit , { digit } , [ "." , digit , { digit } ] ;
```

### 5.3 Semántica de Atributos

- **distancia:** Expresión dimensional. Unidad `px` indica estimación por tamaño de bounding box. Unidades métricas (`m`, `cm`, `ft`) requieren calibración o sensor de profundidad. El sistema abstracta la fuente de profundidad.
- **alerta:** Cadena de texto que se pasará al motor TTS cuando la regla se active.
- **prioridad:** Entero positivo. Menor número = mayor prioridad (similar a `nice`). Se utiliza para resolver colisiones cuando múltiples reglas coinciden en el mismo frame.

---

## 6. Pipeline de Visión en Tiempo Real

### 6.1 Escritorio (Python Generado)

1. **Hilo de Captura:** `cv2.VideoCapture` en hilo dedicado, cola de frames (`queue.Queue`).
2. **Hilo de Inferencia:** Toma frames de la cola, ejecuta ONNX Runtime con modelo YOLOv8-ONNX. Produce bounding boxes, clases, confianzas.
3. **Hilo de Lógica:** Evalúa metadatos contra reglas cargadas. Calcula distancia (inicialmente por tamaño relativo de bounding box normalizada).
4. **Hilo/Callback de Audio:** Si se dispara una regla, encola el mensaje TTS. El audio debe ser no bloqueante para no detener la captura.

### 6.2 Móvil (React Native)

1. **Cámara:** `react-native-vision-camera` o módulo nativo optimizado.
2. **Inferencia:** TFLite en hilo de background (Worklet/Native thread) con modelo YOLOv8-TFLite.
3. **Motor de Reglas:** El código JS generado ejecuta lógica de evaluación contra los resultados de TFLite.
4. **Audio:** Síntesis TTS nativa invocada desde JS cuando se cumplen criterios.

---

## 7. Estructura del Monorepo

```
visioTalk/
├── Cargo.toml                    # Workspace Rust
├── crates/
│   ├── vt-core/                  # AST + Tipos compartidos
│   ├── vt-lexer/                 # Tokenizer (manual o via lalrpop)
│   ├── vt-parser/                # Parser (lalrpop grammar)
│   ├── vt-semantics/             # Analizador semántico
│   ├── vt-codegen/               # Generadores de código (Python, JS)
│   └── vt-cli/                   # CLI del compilador
├── runtime-desktop/
│   ├── pyproject.toml            # uv + Python 3.11+
│   ├── src/
│   │   ├── templates/            # Plantillas base para generación
│   │   ├── visio_engine.py       # Motor de loop de cámara
│   │   └── main.py               # Entrypoint del script generado
│   └── tests/
├── runtime-mobile/
│   ├── android/                  # Configuración nativa Android
│   ├── ios/                      # (Reservado para futuro soporte iOS)
│   ├── src/
│   │   ├── App.tsx               # Entrypoint React Native
│   │   ├── components/
│   │   │   └── Editor.tsx        # Editor de archivos .vt
│   │   └── native/               # Wrappers de TurboModules
│   └── vt-compiler/              # Wrapper del core Rust compilado a lib nativa
├── models/
│   ├── yolov8n.onnx              # Modelo para escritorio (ONNX)
│   └── yolov8n-fp16.tflite       # Modelo para móvil (TFLite)
└── docs/
    ├── grammar.md
    └── api-reference.md
```

---

## 8. Seguridad y Accesibilidad

### 8.1 Seguridad por Diseño

- El análisis semántico es la línea de defensa principal: nada de lo que produzca errores semánticos debe llegar a ejecutarse.
- El DSL no permite código arbitrario: es puramente declarativo, sin loops ni llamadas a funciones externas.
- Validación estricta de rangos para prevenir configuraciones peligrosas (ej. `distancia: < -5m` es inválido).

### 8.2 Accesibilidad

- El aplicativo móvil debe ser 100% operable via TalkBack (Android). Soporte VoiceOver (iOS) se considerará en fases futuras.
- El editor de `.vt` en móvil debe integrar sugerencias auditivas y validación en tiempo real.
- Las alertas TTS deben respetar el volumen del sistema y no saturar al usuario (throttling inteligente).

---

## 9. Requisitos de Rendimiento

| Métrica                     | Escritorio              | Móvil                  |
| --------------------------- | ----------------------- | ---------------------- |
| Latencia cámara → audio     | < 300ms (Linux x86_64)  | < 500ms (Android)      |
| FPS mínimo de procesamiento | 15 FPS                  | 10 FPS                 |
| Precisión de detección      | mAP@50 > 0.35 (YOLOv8n) | mAP@50 > 0.30 (TFLite) |
| Tiempo de compilación .vt   | < 100ms                 | < 500ms (en Android)   |

---

## 10. Milestones

| Fase   | Entregable               | Criterio de Éxito                                                      |
| ------ | ------------------------ | ---------------------------------------------------------------------- |
| Fase 1 | Parser funcional en Rust | Puede parsear el ejemplo de la sección 5.1 sin errores                 |
| Fase 2 | Análisis Semántico + CLI | Detecta errores semánticos (etiquetas inválidas, distancias negativas) |
| Fase 3 | Runtime Escritorio E2E   | Script Python generado ejecuta alertas reales con webcam               |
| Fase 4 | Backend JS + App Android | App carga `.vt`, compila en device Android, genera JS                 |
| Fase 5 | Runtime Android E2E      | App en Android detecta objetos y habla con TFLite                      |

---

## 11. Notas y Dependencias Futuras

- **Sensores de Profundidad:** El DSL ya abstrae la fuente de distancia. Futuras versiones pueden incluir un campo `fuente:` (ej. `fuente: stereo`, `fuente: tof`) en las reglas.
- **Nuevos Targets:** El AST agnóstico permite añadir backends futuros (ej. Web con TensorFlow.js, Edge con Coral TPU).
- **Optimización del Compiler:** Considerar compilación incremental si los archivos `.vt` crecen en complejidad.
