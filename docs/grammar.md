# Gramática Formal del DSL VisioTalk (EBNF)

**Versión:** 1.0  
**Fecha:** 2026-05-18

---

## Sintaxis

```ebnf
program       ::= modo_decl? , rule_decl+ ;
modo_decl     ::= "MODO" , identifier ;
rule_decl     ::= "DETECTAR" , identifier , "{" , attribute* , "}" ;
attribute     ::= key , ":" , value ;
key           ::= "distancia" | "alerta" | "prioridad" ;
value         ::= distance_expr | string_literal | integer_literal ;
distance_expr ::= comparator , numeric_literal , unit ;
comparator    ::= "<" | ">" | "<=" | ">=" | "==" ;
unit          ::= "m" | "cm" | "ft" | "px" ;
identifier    ::= letter , { letter | digit | "_" } ;
string_literal::= '"' , { character - '"' } , '"' ;
numeric_literal::= digit , { digit } , [ "." , digit , { digit } ] ;
letter        ::= "a" | "b" | ... | "z" | "A" | "B" | ... | "Z" ;
digit         ::= "0" | "1" | ... | "9" ;
```

---

## Tokens

| Token                  | Patrón                    | Ejemplo                     |
| ---------------------- | ------------------------- | --------------------------- |
| `MODO`                 | Palabra reservada         | `MODO`                      |
| `DETECTAR`             | Palabra reservada         | `DETECTAR`                  |
| `{` `}`                | Delimitadores de bloque   | `{ ... }`                   |
| `:`                    | Separador clave-valor     | `alerta: "texto"`           |
| `<` `>` `<=` `>=` `==` | Comparadores de distancia | `< 2.0m`                    |
| `m` `cm` `ft` `px`     | Unidades de distancia     | `2.0m`, `150px`             |
| Identificador          | `[a-zA-Z_][a-zA-Z0-9_]*`  | `persona`, `obstaculo_bajo` |
| String literal         | `"[^"]*"`                 | `"Alerta de voz"`           |
| Número literal         | `[0-9]+(\.[0-9]+)?`       | `2.0`, `150`                |
| Espacios y comentarios | `[ \t\n\r]+`, `//.*`      | Ignorados por el lexer      |

---

## Semántica

### Reglas del programa

- Un archivo `.vt` debe contener **al menos una regla `DETECTAR`**.
- La declaración `MODO <nombre>` es opcional y provee contexto semántico (por ahora decorativo).

### Atributo `distancia`

- Expresión comparativa con unidad: `<comparador> <número> <unidad>`.
- La unidad `px` indica estimación de distancia por tamaño relativo de la bounding box (píxeles normalizados).
- Las unidades métricas (`m`, `cm`, `ft`) están reservadas para sensores de profundidad futuros.
- **Rango válido:** `0.0 < valor < 100.0` para cualquier unidad. El analizador semántico debe rechazar valores negativos o cero.

### Atributo `alerta`

- String literal plano. Será sintetizado como voz cuando la regla se active.
- Límite sugerido: 200 caracteres para no saturar el TTS.

### Atributo `prioridad`

- Entero positivo. Menor número = mayor prioridad.
- En caso de que dos reglas disparen simultáneamente, gana la de menor prioridad numérica.
- El analizador semántico debe emitir **advertencia** (no error) si dos reglas tienen la misma prioridad.

---

## Ejemplo Completo

```vt
MODO navegacion_urbana

DETECTAR persona {
    distancia: < 2.0m
    alerta: "Persona cercana"
    prioridad: 1
}

DETECTAR obstaculo {
    distancia: < 0.5m
    alerta: "Obstáculo inmediato"
    prioridad: 0
}

DETECTAR escalera {
    distancia: < 3.0m
    alerta: "Escaleras adelante"
    prioridad: 2
}
```

---

## Restricciones del Analizador Semántico

| Validación                    | Tipo    | Mensaje                                                         |
| ----------------------------- | ------- | --------------------------------------------------------------- |
| Al menos una regla `DETECTAR` | Error   | "El programa debe contener al menos una regla DETECTAR"         |
| Distancia > 0                 | Error   | "La distancia debe ser mayor que 0"                             |
| Distancia < 100               | Warning | "Distancia excede el rango típico del sensor"                   |
| Prioridades duplicadas        | Warning | "Reglas X e Y tienen la misma prioridad — posible colisión"     |
| Etiqueta no reconocida        | Warning | "La etiqueta 'X' no está en el vocabulario conocido del modelo" |
| `alerta` vacía o >200 chars   | Warning | "La alerta está vacía" / "La alerta excede los 200 caracteres"  |
