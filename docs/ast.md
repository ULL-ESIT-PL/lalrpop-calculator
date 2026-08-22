# Explicación del programa src/ast.rs

Véase [src/ast.rs](/src/ast.rs) para el código fuente.

Este archivo define el **AST** (Abstract Syntax Tree, árbol de sintaxis abstracta) para la calculadora.

## 1. El árbol de expresiones (líneas 3-7)

```rust
#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(i32),
    Op(Box<Expr>, OpCode, Box<Expr>),
}
```

- `enum Expr` define un tipo que puede ser **una de estas dos cosas**:
  - `Expr::Number(i32)` — un número entero simple, como `5`.
  - `Expr::Op(Box<Expr>, OpCode, Box<Expr>)` — una operación binaria: una subexpresión izquierda, un operador (`OpCode`), y una subexpresión derecha. Por ejemplo, `2 + 3` sería `Op(Number(2), Add, Number(3))`.

- **¿Por qué `Box<Expr>` y no `Expr` directamente?** Esto es clave en Rust. Un `enum` necesita saber su tamaño en memoria en tiempo de compilación. Pero `Expr` se contiene a sí mismo dentro de `Op` (una expresión puede contener otras expresiones, recursivamente, como en `(2+3)*4`). Si el compilador intentara calcular "¿cuánto ocupa `Expr`?", entraría en un bucle infinito: para saber el tamaño de `Op` necesitaría saber el tamaño de `Expr`, que depende del tamaño de `Op`...

  `Box<T>` rompe ese ciclo: en vez de guardar el `Expr` hijo directamente "dentro" del padre, guarda un **puntero a un `Expr` en el heap**. Un puntero siempre tiene tamaño fijo (8 bytes en 64 bits), así que el compilador ya puede calcular el tamaño de `Expr`. Es el equivalente en Rust a usar un puntero/referencia para nodos de árbol en C o Java.

- `#[derive(Debug, PartialEq)]` es una macro que genera automáticamente:
  - `Debug`: permite imprimir el valor con `{:?}` para depuración.
  - `PartialEq`: permite comparar dos `Expr` con `==`.

## 2. Los operadores (líneas 9-15)

```rust
#[derive(Debug, PartialEq)]
pub enum OpCode {
    Mul,
    Div,
    Add,
    Sub,
}
```

Un enum simple sin datos asociados: solo representa "qué operación es" (multiplicar, dividir, sumar, restar).

## 3. Implementación de `Display` para `Expr` (líneas 17-24)

```rust
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{n}"),
            Expr::Op(lhs, op, rhs) => write!(f, "{op}({lhs}, {rhs})"),
        }
    }
}
```

- `impl fmt::Display for Expr` significa: "estoy implementando el trait `Display` para el tipo `Expr`". Un **trait** en Rust es similar a una interfaz en Java/TypeScript: define un comportamiento (aquí, "cómo convertirte en texto legible"). Al implementarlo, se puede usar `{}` (en vez de `{:?}`) para imprimir un `Expr`, y también funciona automáticamente `to_string()`.

- `match self { ... }` es un **pattern matching** sobre las variantes del enum:
  - Si es `Expr::Number(n)`, escribe simplemente el número.
  - Si es `Expr::Op(lhs, op, rhs)`, escribe algo como `Add(2, 3)` — nota que esto es **recursivo**: el `{lhs}` y `{rhs}` dentro del `write!` llaman de nuevo a `Display` sobre las subexpresiones (que a su vez pueden ser `Number` u otro `Op`), permitiendo imprimir árboles anidados como `Mul(Add(2, 3), 4)`.

- Curiosidad: aunque `lhs` y `rhs` son `Box<Expr>`, dentro del `write!` se usan como si fueran `Expr` directamente (`{lhs}`). Esto es porque `Box<T>` implementa "auto-deref": Rust deja usarlo casi transparentemente como si fuera el valor de dentro.

## 4. Implementación de `Display` para `OpCode` (líneas 26-36)

Lo mismo, pero para el operador: convierte cada variante (`Mul`, `Div`, `Add`, `Sub`) en su nombre como texto (`"Mul"`, `"Div"`, etc.).

---

**Resumen:** este código modela expresiones aritméticas como un árbol recursivo (`Expr`), usa `Box` para permitir esa recursión en memoria, y define cómo convertir ese árbol en texto legible mediante el trait `Display`, que es lo que probablemente use la gramática LALRPOP para imprimir el resultado del parseo.
