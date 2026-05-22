# Stack Tecnológico Consolidado - Proyecto VisioTalk

**Última actualización:** 2026-05-18  
**Alcance:** Android exclusivo (runtime móvil) + Escritorio Linux exclusivo

---

## 1. Compilador Core (Rust)

El núcleo del compilador se implementa en Rust para garantizar rendimiento nativo, seguridad de memoria y la capacidad de compilar a bibliotecas nativas para Android.

| Componente                | Crate / Herramienta    | Versión Recomendada    | Propósito                                          |
| ------------------------- | ---------------------- | ---------------------- | -------------------------------------------------- |
| Lenguaje                  | `rustc` + `cargo`      | Última estable (1.78+) | Compilación y gestión de dependencias              |
| Lexer                     | `logos`                | `^0.14`                | Tokenización DFA de alto rendimiento               |
| Parser                    | `lalrpop`              | `^0.20`                | Generación de parser LR(1) desde gramática EBNF    |
| AST / Serialización       | `serde` + `serde_json` | `^1.0`                 | Representación intermedia y debug                  |
| Templates                 | `askama`               | `^0.12`                | Generación type-safe de código Python/JS           |
| CLI                       | `clap` (derive)        | `^4.5`                 | Interfaz de línea de comandos (`vt compile`)       |
| Errores / Diagnósticos    | `ariadne`              | `^0.4`                 | Reportes de error semántico con subrayado          |
| Python FFI                | `pyo3`                 | `^0.21`                | Exponer el compilador como librería Python         |
| Build Python wheels       | `maturin`              | `^1.5`                 | Empaquetado del core Rust para `pip`/`uv`          |
| Cross-compilación Android | `cargo-ndk`            | `^3.4`                 | Compilar Rust a `.so` para Android (arm64, x86_64) |
| Testing                   | `insta`                | `^1.39`                | Snapshot testing del AST y outputs generados       |
| Linting                   | `clippy`               | Incluido en toolchain  | Análisis estático de código Rust                   |
| Formateo                  | `rustfmt`              | Incluido en toolchain  | Formateo automático                                |

### Dependencias del Build (Cargo.toml)

```toml
[workspace]
members = [
    "crates/vt-core",
    "crates/vt-lexer",
    "crates/vt-parser",
    "crates/vt-semantics",
    "crates/vt-codegen",
    "crates/vt-cli"
]
resolver = "2"

[workspace.dependencies]
logos = "0.14"
lalrpop = "0.20"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
askama = "0.12"
clap = { version = "4.5", features = ["derive"] }
ariadne = "0.4"
pyo3 = { version = "0.21", features = ["extension-module"] }
insta = "1.39"
```

---

## 2. Runtime Escritorio (Python)

El script Python generado por el compilador se ejecuta en escritorio y gestiona el pipeline completo de cámara → visión computacional → TTS.

| Componente         | Paquete / Herramienta   | Versión Recomendada | Propósito                                       |
| ------------------ | ----------------------- | ------------------- | ----------------------------------------------- |
| Lenguaje           | Python                  | `3.11+`             | Ejecución del script transpilado                |
| Gestor de paquetes | `uv`                    | Última estable      | Reemplazo rápido a pip/poetry/venv              |
| Captura de video   | `opencv-python`         | `^4.9`              | Adquisición de frames desde webcam              |
| Inferencia IA      | `onnxruntime`           | `^1.17`             | Ejecución agnóstica de modelos ONNX             |
| Modelo base        | `ultralytics`           | `^8.2`              | YOLOv8-nano, exportación a ONNX                 |
| Síntesis de voz    | `pyttsx3`               | `^2.90`             | TTS offline (requiere `espeak-ng` en Linux)     |
| Concurrencia       | `asyncio` / `threading` | Estándar            | Pipeline paralelo de cámara, inferencia y audio |
| Linting            | `ruff`                  | `^0.4`              | Linter y formateador ultrarrápido               |
| Type checking      | `mypy`                  | `^1.10`             | Verificación estática de tipos                  |

### Archivo de Configuración (pyproject.toml)

```toml
[project]
name = "visiotalk-runtime"
version = "0.1.0"
description = "Runtime de escritorio para VisioTalk"
requires-python = ">=3.11"
dependencies = [
    "opencv-python>=4.9.0",
    "onnxruntime>=1.17.0",
    "ultralytics>=8.2.0",
    "pyttsx3>=2.90",
]

[project.optional-dependencies]
dev = [
    "ruff>=0.4.0",
    "mypy>=1.10.0",
    "pytest>=8.0.0",
]

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

---

## 3. Runtime Móvil (React Native + Android Nativo)

La aplicación móvil se desarrolla en React Native con módulos nativos para integrar el compilador Rust y ejecutar inferencia con TensorFlow Lite.

| Componente      | Paquete / Herramienta                       | Versión Recomendada         | Propósito                              |
| --------------- | ------------------------------------------- | --------------------------- | -------------------------------------- |
| Framework       | React Native                                | `0.74+`                     | UI y lógica de la aplicación           |
| Lenguaje        | TypeScript                                  | `5.4+`                      | Tipado estático para el código RN      |
| Cámara          | `react-native-vision-camera`                | `^4.0`                      | Captura de frames eficiente en Android |
| Inferencia IA   | `react-native-tflite` / Custom TurboModule  | Dependiente                 | Ejecutar modelo TFLite en background   |
| Modelo base     | YOLOv8-nano (TFLite)                        | Exportado desde Ultralytics | Detección de objetos optimizada        |
| Síntesis de voz | `react-native-tts`                          | `^4.1`                      | TTS nativo de Android                  |
| Storage         | `@react-native-async-storage/async-storage` | `^1.23`                     | Guardar archivos `.vt` y preferencias  |
| FileSystem      | `react-native-fs`                           | `^2.20`                     | Lectura/escritura de archivos `.vt`    |
| Navigation      | `@react-navigation/native`                  | `^6.1`                      | Navegación entre pantallas             |
| Linting         | ESLint + Prettier                           | Config estándar RN          | Calidad de código TS/JS                |

### Dependencias de Desarrollo (package.json)

```json
{
  "dependencies": {
    "react": "18.2.0",
    "react-native": "0.74.0",
    "react-native-vision-camera": "^4.0.0",
    "react-native-tts": "^4.1.0",
    "@react-native-async-storage/async-storage": "^1.23.0",
    "react-native-fs": "^2.20.0",
    "@react-navigation/native": "^6.1.0",
    "@react-navigation/stack": "^6.3.0"
  },
  "devDependencies": {
    "@types/react": "^18.2.0",
    "typescript": "^5.4.0",
    "eslint": "^8.57.0",
    "prettier": "^3.2.0"
  }
}
```

---

## 4. DevOps, CI/CD y Control de Calidad

| Herramienta               | Propósito                          | Integración                            |
| ------------------------- | ---------------------------------- | -------------------------------------- |
| **Git**                   | Control de versiones               | Monorepo local y remoto                |
| **GitHub Actions**        | CI/CD automatizado                 | Tests, builds, linting en cada push/PR |
| **Cargo audit**           | Escaneo de vulnerabilidades        | `cargo install cargo-audit`            |
| **Dependabot / Renovate** | Actualización automática de deps   | Configurar en repositorio GitHub       |
| **Act**                   | Ejecutar GitHub Actions localmente | `act -j test` para validar workflows   |

### GitHub Actions Workflows Propuestos

```yaml
# .github/workflows/ci.yml
name: CI
on: [push, pull_request]
jobs:
  rust:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-action@stable
      - run: cargo test --workspace
      - run: cargo clippy -- -D warnings
      - run: cargo fmt --check

  python:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: astral-sh/setup-uv@v1
      - run: uv pip install -e runtime-desktop/
      - run: cd runtime-desktop && ruff check . && mypy src/

  android:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-java@v4
        with:
          java-version: "17"
          distribution: "temurin"
      - run: cd runtime-mobile/android && ./gradlew assembleDebug
```

---

## 5. Hardware / Targets Soportados

| Plataforma                 | Arquitecturas | Notas                                                |
| -------------------------- | ------------- | ---------------------------------------------------- |
| **Escritorio Linux**       | `x86_64`      | ONNX Runtime detecta CUDA/OpenVINO automáticamente   |
| **Android (Dispositivos)** | `arm64-v8a`   | Core Rust como `.so`, TFLite acelerado por NNAPI/GPU |
| **Android (Emulador)**     | `x86_64`      | Para testing sin dispositivo físico                  |

### Compilación Cruzada Android (cargo-ndk)

```bash
# Instalar cargo-ndk
cargo install cargo-ndk

# Compilar core Rust para Android (arm64 + x86_64)
cargo ndk -t arm64-v8a -t x86_64 -o ./runtime-mobile/android/app/src/main/jniLibs build --release
```

---

## 6. Modelos de IA y Formatos

| Uso        | Modelo      | Framework   | Formato       | Exportación                                       |
| ---------- | ----------- | ----------- | ------------- | ------------------------------------------------- |
| Escritorio | YOLOv8-nano | Ultralytics | ONNX          | `yolo export model=yolov8n.pt format=onnx`        |
| Android    | YOLOv8-nano | Ultralytics | TFLite (FP16) | `yolo export model=yolov8n.pt format=tflite half` |

---

## 7. Flujo de Trabajo del Compilador

```
Archivo .vt (input del usuario)
                │
                ▼
┌─────────────────────────────────────┐
│         vt-cli (Rust)               │
│  ┌─────────────────────────────┐    │
│  │  logos (Lexer)              │    │
│  │  ↓                          │    │
│  │  lalrpop (Parser)           │    │
│  │  ↓                          │    │
│  │  Semántica (Rust nativo)    │    │
│  │  ↓                          │    │
│  │  AST Validado               │    │
│  └─────────────────────────────┘    │
│  ┌─────────────────────────────┐    │
│  │  askama (Templates)         │    │
│  │  ↓                          │    │
│  │  Python / JavaScript        │    │
│  └─────────────────────────────┘    │
└──────────────┬──────────────────────┘
               │
       ┌───────┴───────┐
       ▼               ▼
   Script .py      Módulo .js
       │               │
       ▼               ▼
┌──────────────┐  ┌─────────────────────────┐
│   Desktop    │  │      Android (RN)       │
│  ┌────────┐  │  │  ┌───────────────────┐  │
│  │ OpenCV │  │  │  │ vision-camera     │  │
│  │ ONNX   │  │  │  │ TFLite (JNI)      │  │
│  │ pyttsx3│  │  │  │ react-native-tts  │  │
│  └────────┘  │  │  └───────────────────┘  │
└──────────────┘  └─────────────────────────┘
```

---

## 8. Instalación Rápida del Entorno

### Rust (Compilador Core)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup component add rustfmt clippy
cargo install cargo-ndk maturin cargo-audit
```

### Python (Runtime Escritorio)

```bash
curl -LsSf https://astral.sh/uv/install.sh | sh
uv venv --python 3.11
source .venv/bin/activate
uv pip install -e runtime-desktop/
```

### React Native (Runtime Android)

```bash
npx react-native@latest init VisioTalkMobile
cd VisioTalkMobile
npm install react-native-vision-camera react-native-tts \
  @react-native-async-storage/async-storage react-native-fs \
  @react-navigation/native @react-navigation/stack
```
