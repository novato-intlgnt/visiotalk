# DOCUMENTO DE CONTEXTO TÉCNICO: PROYECTO VISIOTALK

## 1. DESCRIPCIÓN GENERAL DEL PROYECTO

**VisioTalk** es un ecosistema de software centrado en un **Lenguaje de Dominio Específico (DSL)** diseñado para la asistencia y navegación independiente de personas con discapacidad visual.

El núcleo del proyecto es un **compilador/transpiler** que toma un archivo de configuración declarativo y simple de alto nivel (el código fuente de VisioTalk) y lo traduce a una implementación optimizada y de bajo nivel que integra flujos de vídeo en tiempo real, inferencia con modelos de Inteligencia Artificial (Visión Computacional) y síntesis de voz (Text-to-Speech) para generar alertas auditivas espaciales o concisas.

### Objetivos Principales:

- **Abstracción de Complejidad:** Separar la lógica de asistencia (el "qué" se quiere detectar y bajo qué umbrales) de la fontanería técnica y pesada de la IA (el "cómo" interactúan las APIs de captura de video, los tensores y los hilos de audio).
- **Seguridad por Diseño:** Utilizar las fases de análisis estático del compilador (análisis léxico, sintáctico y semántico) para validar formalmente las reglas de navegación antes de la ejecución, previniendo fallos críticos o configuraciones peligrosas en entornos reales.
- **Portabilidad y Optimización:** Diseñar un lenguaje agnóstico que pueda ser compilado a un script standalone optimizado para entornos de escritorio (como Linux de bajo nivel) o interpretado/ejecutado nativamente mediante un motor JIT en dispositivos móviles utilizando formatos de inferencia ligera.

---

## 2. ARQUITECTURA DEL COMPILADOR (EL CORE)

El pipeline de compilación de VisioTalk sigue las fases clásicas de la teoría de lenguajes y compiladores:

1. **Análisis Léxico (Lexer / Scanner):** Convierte el flujo de caracteres del archivo de entrada (`.vt`) en una secuencia de tokens válidos. Identifica palabras clave como `DETECTAR`, `MODO`, identificadores de atributos (`distancia:`, `alerta:`, `prioridad:`), literales (cadenas de texto, números flotantes) y delimitadores.
2. **Análisis Sintáctico (Parser):** Evalúa la secuencia de tokens frente a una gramática formal estructurada en notación **EBNF**. Construye un **Árbol de Sintaxis Abstracta (AST)** que representa la estructura jerárquica de las reglas declaradas por el usuario.
3. **Análisis Semántico:** Es la capa crítica de seguridad del proyecto. Valida las reglas de negocio y restricciones físicas antes de generar código executable. Por ejemplo:

- Verifica que los objetos a detectar pertenezcan al espacio de etiquetas válidas del modelo de IA subyacente.
- Valida rangos lógicos de distancia (ej. rechazar distancias negativas o fuera del rango focal del sensor).
- Previene colisiones o contradicciones entre reglas paralelas de una misma prioridad.

4. **Generación de Código / Motor de Ejecución:**

- _Modo Escritorio (Transpiler):_ Traduce el AST utilizando un motor de plantillas (tipo Jinja2) para inyectar la lógica de las reglas dentro de un bucle infinito optimizado de captura de video e inferencia de visión computacional, exportando un archivo ejecutable (ej. `.py`).
- _Modo Móvil:_ Transforma el AST en una estructura intermedia estructurada (JSON/Binario) que es interpretada en tiempo real por el motor de inferencia interno del aplicativo móvil.

---

## 3. ARQUITECTURA DEL PIPELINE EN TIEMPO REAL (DE LA CÁMARA AL USUARIO)

Cuando el código generado o interpretado se ejecuta, el ciclo de retroalimentación asíncrono opera de la siguiente manera:

1. **Captura de Video (Input):** Adquisición continua de frames desde el sensor de la cámara del dispositivo, manejada idealmente en un hilo de ejecución independiente para evitar cuellos de botella en la interfaz.
2. **Inferencia de Visión Computacional (Procesamiento):** El frame pasa por un modelo ligero de detección de objetos. El modelo genera como salida cajas delimitadoras (_bounding boxes_), etiquetas de clases y porcentajes de confianza.
3. **Evaluación de Reglas Semánticas (Lógica de Negocio):** Se procesan los metadatos de los objetos detectados frente a las reglas cargadas en memoria (ej. estimación de distancia basada en el tamaño relativo de la bounding box o profundidad si el sensor lo permite).
4. **Motor de Alerta Auditiva (Output):** Si se cumplen los criterios de una regla, se despacha un evento al sistema de síntesis de voz (TTS). Este módulo requiere un diseño concurrente estricto: el audio debe ser no bloqueante para que el pipeline de la cámara siga procesando el entorno en tiempo real mientras el usuario escucha la alerta.

---

## 4. RESPALDO CIENTÍFICO E INVESTIGACIÓN

El desarrollo de este proyecto se fundamenta en principios extraídos de literatura indexada y peer-reviewed:

- _Modelado dirigido por DSL e interacciones de IA (Smyth et al., 2025):_ Soporta la necesidad de integrar transparencia y verificación formal del código generado utilizando técnicas de modelado específicas.
- _Ingeniería de Requisitos para Redes Neuronales vía DSL (Jahic et al., 2023):_ Valida el enfoque de usar un lenguaje formalizado para especificar con precisión las capacidades y alcances de reconocimiento esperados en componentes basados en IA.
- _Optimización de Compiladores en Visión Computacional:_ Precedentes como _HipaccVX_ demuestran que los entornos basados en DSLs permiten inyectar optimizaciones automáticas de hardware que superan las implementaciones manuales genéricas.

---

## INSTRUCCIONES PARA OPENCODE

_(Pasa este bloque de instrucciones directamente a la IA de desarrollo junto con el contexto de arriba)_

> **Mensaje para OpenCode:**
> Has recibido el contexto completo del proyecto **VisioTalk**. Tu objetivo es actuar como el arquitecto técnico principal para coordinar y estructurar el entorno de desarrollo. Siguiendo las directrices de diseño limpio (**Clean Architecture**), herramientas basadas en terminales eficientes de desarrollo y entornos optimizados, debes tomar la iniciativa.
> **Por favor, analiza el contexto provisto y hazme una serie de preguntas críticas y específicas para consolidar:**
>
> 1. El stack tecnológico definitivo para el compilador core (Frontend del compilador: herramientas para el Lexer y Parser).
> 2. El motor de plantillas o estrategia de generación/transpilación de código para el backend.
> 3. El stack de visión artificial y formatos de optimización de modelos (especialmente pensando en la dualidad Escritorio/Móvil).
> 4. La arquitectura del aplicativo móvil y la estrategia para portar o interpretar el lenguaje VisioTalk on-device.
> 5. El setup del entorno de desarrollo (herramientas de testing, linters y control de versiones).
>
> Formula preguntas precisas que me ayuden a tomar las decisiones técnicas correctas para crear un correcto PRD acerca del proyecto y creemos un plan de implementacion de acuerdo a las fases indicadas.
