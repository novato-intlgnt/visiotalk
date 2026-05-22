# Preguntas Críticas para la Consolidación Técnica de VisioTalk

## 1. Stack del Compilador Core (Lexer y Parser)

**¿Cuál es el lenguaje de implementación preferido para el núcleo del compilador?**

- [x] **Rust**: Máximo rendimiento y seguridad de memoria. Ideal si el compilador podría embeberse en el móvil más adelante.
- [x] **C++**: Rendimiento extremo, pero mayor complejidad de desarrollo y menos ecosistema moderno de parsing.

**¿Prefieres un generador de parsers automático o una implementación manual?**

- [x] **Generador automático (ANTLR4, PEG, Yacc/Bison)**: Rápido de levantar, gramática declarativa, pero menos control sobre los mensajes de error.

## 2. Motor de Plantillas / Estrategia de Generación de Código (Backend)

**¿Cuál es el target de ejecución para el modo Escritorio?**

- [x] **Python**: Alto nivel, ecosistema de IA maduro (OpenCV, PyTorch, TTS), pero requiere un intérprete en el target.

**¿Cómo se ejecutará la lógica de VisioTalk en el dispositivo Móvil?**

- El Enfoque Híbrido (Transpilación a JS + Arquitectura Orientada a Eventos)
  Para VisioTalk, la mejor alternativa es que tu compilador de escritorio transpile la lógica del lenguaje a JavaScript estándar (ES6), estructurado bajo un modelo de eventos.
  ¿Cómo funcionaría la arquitectura?
  - En el Escritorio (Compilador): El usuario diseña la lógica de asistencia visual. Tu compilador traduce esa sintaxis a un archivo JS estructurado con hooks específicos (ej. onObjectDetected(obj) { ... }).

  - En el Móvil (React Native): La app expone los servicios nativos de Android (cámara, modelo de IA/TFLite, Text-to-Speech) a través de un contexto global o API que el script generado puede invocar.

## 3. Stack de Visión Artificial y Optimización de Modelos

**¿Qué familia de modelos de detección de objetos prefieres como base?**

- [x] **YOLO (Ultralytics)**: Estado del arte en velocidad/precisión, muy maduro en Python, con exportación a ONNX, CoreML y TFLite.

**¿Cuál es el hardware objetivo para la inferencia en Escritorio?**

- [x] **Agnóstico (ONNX Runtime con Execution Providers)**: Detecta automáticamente CUDA, DirectML, OpenVINO, etc.

**¿Qué frameworks o formatos contemplas para la inferencia en Móvil?**

- [x] **TensorFlow Lite (TFLite)**: El estándar para Android, con soporte de aceleradores (GPU, NNAPI).

**¿Cómo se calculará la distancia en el DSL?**

- [x] **Abstracción para profundidad**: El DSL debe permitir futuras fuentes de profundidad (ToF, LiDAR, cámara stereo) aunque ahora se use solo la bounding box.

## 4. Arquitectura del Aplicativo Móvil y Estrategia On-Device

**¿Qué framework prefieres para el desarrollo del aplicativo móvil?**

- [x] **React Native**: Ecosistema grande, pero tradicionalmente menos performante para procesamiento de cámara en tiempo real, aunque mas importante es el compiladorr y las pruebas.

**¿El flujo de trabajo para el usuario móvil será...?**

- [x] **Nativo en Móvil**: La app móvil contiene un editor y un mini-compilador para cargar `.vt` directamente sin necesidad de PC.

## 5. Setup del Entorno de Desarrollo

**¿Cómo prefieres estructurar el repositorio?**

- [x] **Monorepo**: Todo el proyecto (compilador, runtime de escritorio, app móvil) en un solo repositorio para facilitar la coherencia de versiones.

**¿Cuál es la versión mínima de Python para el ecosistema de escritorio?**

- [x] **Python 3.11+**: Mejoras de rendimiento en el intérprete, buen balance entre modernidad y compatibilidad, ademas si se necesita algun instalador de paquetes para python usaremos uv.
