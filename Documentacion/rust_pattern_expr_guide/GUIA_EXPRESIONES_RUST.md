# Guía Completa: Expresiones en Rust y Extract Dynamic Elements

## Tabla de Contenidos
1. [Conceptos Fundamentales de Expresiones](#conceptos-fundamentales)
2. [La Librería `syn`](#librería-syn)
3. [Análisis de `extract_dynamic_elements`](#análisis-extract-dynamic-elements)
4. [Patrones Soportados](#patrones-soportados)
5. [Mejoras con Expresiones Regulares](#mejoras-regex)
6. [Ejemplos Prácticos](#ejemplos-prácticos)

---

## Conceptos Fundamentales de Expresiones

### ¿Qué es una Expresión en Rust?

Una **expresión** es un fragmento de código que **devuelve un valor**. A diferencia de las sentencias, que terminan con `;` y no retornan nada.

```rust
// EXPRESIÓN (sin ;)
let x = 5 + 6;  // 5 + 6 es una expresión que evalúa a 11

// SENTENCIA (con ;)
let x = 5 + 6;  // La línea completa es una sentencia

// DIFERENCIA CLAVE:
let y = (let x = 3);    // ❌ ERROR: sentencia dentro de expresión
let z = { let x = 3; }; // ✅ OK: bloque (expresión)
```

### Tipos de Expresiones en Rust

| Tipo | Ejemplo | Resultado |
|------|---------|-----------|
| **Aritmética** | `5 + 6` | `11` |
| **Comparación** | `5 > 4` | `true` |
| **Lógica** | `true && false` | `false` |
| **Acceso** | `array[0]` | Elemento |
| **Llamada** | `func()` | Valor retornado |
| **Dereference** | `*ptr` | Valor apuntado |
| **Referencia** | `&x` | Dirección de x |
| **Bloque** | `{ x + 1 }` | Último valor |
| **Unsafe** | `unsafe { ... }` | Resultado interno |

### Estructura AST de una Expresión

El **Árbol de Sintaxis Abstracta (AST)** representa la estructura jerárquica de una expresión:

```
Expresión: *ptr + 5
├── BinaryOp (+)
│   ├── Left: UnaryOp (Deref)
│   │   └── Expr: Path (ptr)
│   └── Right: Lit (5)
```

---

## Librería `syn`

### ¿Qué es `syn`?

`syn` es una librería que **parsea código Rust en un AST estructurado** y proporciona tipos para trabajar con cada elemento sintáctico.

### Estructura de Tipos en `syn`

```rust
// Nivel superior
pub enum Expr {
    Binary(ExprBinary),      // a + b
    Unary(ExprUnary),        // *ptr, -x, !b
    Unsafe(ExprUnsafe),      // unsafe { ... }
    Path(ExprPath),          // variable, module::item
    Call(ExprCall),          // function()
    Index(ExprIndex),        // array[0]
    Reference(ExprReference),// &x, &mut x
    RawAddr(ExprRawAddr),    // &raw const x
    Assign(ExprAssign),      // x = 5
    // ... y muchas más
}

// Expresión Unaria (operador + valor)
pub struct ExprUnary {
    pub op: UnOp,            // El operador
    pub expr: Box<Expr>,     // El operando
}

pub enum UnOp {
    Deref(_),   // * (dereference)
    Not(_),     // ! (negación lógica)
    Neg(_),     // - (negación aritmética)
}

// Bloque Unsafe
pub struct ExprUnsafe {
    pub unsafe_token: Unsafe,
    pub block: Block,        // { statements }
}

pub struct Block {
    pub stmts: Vec<Stmt>,    // Sentencias dentro
}
```

### Visitor Pattern (Patrón Visitante)

`syn` usa el **patrón visitante** para recorrer el AST:

```rust
// Visitor: solo lectura
trait Visit<'ast> {
    fn visit_expr(&mut self, node: &'ast Expr) { ... }
    fn visit_expr_unsafe(&mut self, node: &'ast ExprUnsafe) { ... }
}

// VisitMut: lectura y escritura (modificación)
trait VisitMut {
    fn visit_expr_mut(&mut self, node: &mut Expr) { ... }
    fn visit_expr_unsafe_mut(&mut self, node: &mut ExprUnsafe) { ... }
}
```

---

## Análisis de `extract_dynamic_elements`

### Propósito General

La función **extrae variables y valores clave de un bloque `unsafe`** según su patrón de uso. Esto permite **reutilizar estos elementos en templates de reemplazo**.

```rust
fn extract_dynamic_elements(expr_unsafe: &ExprUnsafe, pattern_kind: &str) -> HashMap<String, String>
```

**Entrada:**
- `expr_unsafe`: El bloque unsafe a analizar
- `pattern_kind`: El tipo de patrón detectado (ej: "deref_expr")

**Salida:**
- `HashMap<String, String>`: Mapeo de placeholders (`{var}`, `{expr}`) a valores reales

### Flujo de Trabajo

```
expr_unsafe
    ↓
match pattern_kind {
    "deref_expr" → Busca *var → Extrae "var"
    "assign_to_deref" → Busca *ptr = value → Extrae "var" y "expr"
    "raw_addr_expr" → Busca &raw const/mut var → Extrae "var"
    _ → Extrae la primera expresión del bloque
}
    ↓
HashMap { "var": "ptr_name", "expr": "value_expr" }
```

### Análisis Detallado por Patrón

#### 1. **Patrón: "deref_expr" (Dereference)**

**Código original:**
```rust
unsafe {
    *ptr
}
```

**AST:**
```
ExprUnsafe {
    block: Block {
        stmts: [
            Stmt::Expr(
                Expr::Unary(
                    ExprUnary {
                        op: UnOp::Deref(_),
                        expr: Box::new(Expr::Path(ExprPath { path: "ptr" }))
                    }
                )
            )
        ]
    }
}
```

**Proceso de extracción:**

```rust
// Paso 1: Itera sobre stmts
for stmt in &block.stmts {
    
    // Paso 2: Pattern matching - verifica si es Expr::Unary con Deref
    if let syn::Stmt::Expr(Expr::Unary(ExprUnary { 
        op: UnOp::Deref(_),  // ← Verifica que sea *
        expr,                 // ← Captura el operando
        .. 
    }), _) = stmt {
        
        // Paso 3: Verifica que el operando sea una variable (Path)
        if let Expr::Path(ExprPath { path, .. }) = &**expr {
            
            // Paso 4: Extrae el identificador
            if let Some(ident) = path.get_ident() {
                elements.insert("var".to_string(), ident.to_string());
                // "var" → "ptr"
            }
        }
    }
}
```

**Resultado:**
```rust
HashMap { "var": "ptr" }

// Luego se usa en template:
// Template: "Box::new({var})"
// Reemplazo: "Box::new(ptr)"
```

#### 2. **Patrón: "assign_to_deref" (Asignación a Puntero)**

**Código original:**
```rust
unsafe {
    *ptr = 42
}
```

**AST:**
```
ExprUnsafe {
    block: Block {
        stmts: [
            Stmt::Expr(
                Expr::Assign(ExprAssign {
                    left: Box::new(Expr::Unary(
                        ExprUnary {
                            op: UnOp::Deref(_),
                            expr: Box::new(Expr::Path { path: "ptr" })
                        }
                    )),
                    right: Box::new(Expr::Lit(ExprLit { lit: 42 }))
                })
            )
        ]
    }
}
```

**Proceso de extracción:**

```rust
for stmt in &block.stmts {
    // Paso 1: Detecta asignación
    if let syn::Stmt::Expr(Expr::Assign(assign), _) = stmt {
        
        // Paso 2: Verifica que la izquierda sea *var
        if let Expr::Unary(ExprUnary { 
            op: UnOp::Deref(_), 
            expr, 
            .. 
        }) = &*assign.left {
            
            // Paso 3: Extrae el identificador del puntero
            if let Expr::Path(ExprPath { path, .. }) = &**expr {
                if let Some(ident) = path.get_ident() {
                    elements.insert("var".to_string(), ident.to_string());
                    // "var" → "ptr"
                }
            }
        }
        
        // Paso 4: Extrae el lado derecho (valor asignado)
        elements.insert("expr".to_string(), 
                       assign.right.to_token_stream().to_string());
        // "expr" → "42"
    }
}
```

**Resultado:**
```rust
HashMap { 
    "var": "ptr",
    "expr": "42"
}

// Template: "mem::replace({var}, {expr})"
// Reemplazo: "mem::replace(ptr, 42)"
```

#### 3. **Patrón: "raw_addr_expr" (Dirección Bruta)**

**Código original:**
```rust
unsafe {
    &raw const x
}
```

**AST:**
```
ExprUnsafe {
    block: Block {
        stmts: [
            Stmt::Expr(
                Expr::RawAddr(ExprRawAddr {
                    raw_token: Raw,
                    mutability: None,  // const
                    expr: Box::new(Expr::Path { path: "x" })
                })
            )
        ]
    }
}
```

**Proceso:**

```rust
for stmt in &block.stmts {
    if let syn::Stmt::Expr(Expr::RawAddr(raw_addr), _) = stmt {
        // Simplemente convierte la expresión a string
        elements.insert("var".to_string(), 
                       raw_addr.expr.to_token_stream().to_string());
        // "var" → "x"
    }
}
```

#### 4. **Patrón por defecto: Otros patrones**

Para patrones desconocidos o genéricos:

```rust
_ => {
    if !block.stmts.is_empty() {
        let first_stmt = &block.stmts[0];
        // Extrae la primera expresión completa
        if let syn::Stmt::Expr(expr, _) = first_stmt {
            elements.insert("expr".to_string(), 
                           expr.to_token_stream().to_string());
        }
    }
}
```

---

## Patrones Soportados

| Patrón | Ejemplo | Extrae |
|--------|---------|--------|
| **deref_expr** | `unsafe { *ptr }` | `var` = "ptr" |
| **assign_to_deref** | `unsafe { *ptr = 42 }` | `var` = "ptr", `expr` = "42" |
| **raw_addr_expr** | `unsafe { &raw const x }` | `var` = "x" |
| **raw_pointer_type** | `unsafe { *const T }` | Tipo |
| **binary_arith_expr** | `unsafe { a + b }` | `expr` = "a + b" |
| **array_index_expr** | `unsafe { arr[0] }` | `expr` = "arr[0]" |
| **mutable_ref_expr** | `unsafe { &mut x }` | `expr` = "&mut x" |
| **unsafe_block** | `unsafe { ... }` | Contenido completo |

---

## Mejoras con Expresiones Regulares

### Problema Actual

La función actual **solo maneja patrones específicos y pre-definidos**. Si aparece un patrón no previsto, falla silenciosamente.

### Mejora 1: Extracción Flexible con Regex

```rust
use regex::Regex;

fn extract_variables_with_regex(code: &str) -> Vec<String> {
    // Detecta identificadores válidos en Rust
    let re = Regex::new(r"\b([a-zA-Z_][a-zA-Z0-9_]*)\b").unwrap();
    
    re.captures_iter(code)
        .map(|cap| cap[1].to_string())
        .collect()
}

// Ejemplo:
let code = "*ptr + offset";
let vars = extract_variables_with_regex(code);
// vars = ["ptr", "offset"]
```

### Mejora 2: Clasificación Automática de Patrones

```rust
fn classify_pattern_with_regex(code: &str) -> String {
    let patterns = vec![
        (r"^\*[a-zA-Z_]\w*$", "simple_deref"),
        (r"^\*[a-zA-Z_]\w*\s*=\s*.+$", "assign_to_deref"),
        (r"^&raw\s+(const|mut)\s+[a-zA-Z_]\w*$", "raw_addr"),
        (r"^\w+\[\d+\]$", "array_index"),
        (r"^&mut\s+\w+$", "mutable_ref"),
    ];
    
    for (regex_str, label) in patterns {
        if let Ok(re) = Regex::new(regex_str) {
            if re.is_match(code) {
                return label.to_string();
            }
        }
    }
    "unknown".to_string()
}
```

### Mejora 3: Extracción Estructurada

```rust
use regex::Regex;
use std::collections::HashMap;

fn smart_extract_elements(code: &str) -> HashMap<String, Vec<String>> {
    let mut result = HashMap::new();
    
    // Variables
    let var_re = Regex::new(r"\b([a-zA-Z_][a-zA-Z0-9_]*)\b").unwrap();
    let vars: Vec<String> = var_re.captures_iter(code)
        .map(|c| c[1].to_string())
        .collect();
    result.insert("variables".to_string(), vars);
    
    // Literales números
    let num_re = Regex::new(r"\b(\d+)\b").unwrap();
    let nums: Vec<String> = num_re.captures_iter(code)
        .map(|c| c[1].to_string())
        .collect();
    result.insert("numbers".to_string(), nums);
    
    // Operadores
    let op_re = Regex::new(r"([+\-*/%])").unwrap();
    let ops: Vec<String> = op_re.captures_iter(code)
        .map(|c| c[1].to_string())
        .collect();
    result.insert("operators".to_string(), ops);
    
    result
}

// Ejemplo:
let code = "*ptr + 42 - offset";
let elements = smart_extract_elements(code);
// {
//     "variables": ["ptr", "offset"],
//     "numbers": ["42"],
//     "operators": ["+", "-"]
// }
```

### Mejora 4: Fusión syn + Regex (Enfoque Híbrido)

La **mejor estrategia** es combinar ambos:

```rust
use regex::Regex;

fn extract_dynamic_elements_v2(
    expr_unsafe: &ExprUnsafe,
    pattern_kind: &str,
) -> HashMap<String, String> {
    let mut elements = HashMap::new();
    let block = &expr_unsafe.block;
    let code_str = format!("{}", quote! { #block });
    
    // Primero, intenta con syn (más preciso)
    match pattern_kind {
        "deref_expr" => {
            // ... código syn existente ...
        }
        _ => {
            // Si syn no coincide, usa regex como fallback
            let re_vars = Regex::new(r"\b([a-zA-Z_][a-zA-Z0-9_]*)\b").unwrap();
            let vars: Vec<String> = re_vars.captures_iter(&code_str)
                .map(|c| c[1].to_string())
                .collect();
            
            if !vars.is_empty() {
                elements.insert("variables".to_string(), vars.join(", "));
            }
            
            // Extrae toda la expresión
            elements.insert("expr".to_string(), code_str.trim().to_string());
        }
    }
    
    elements
}
```

---

## Ejemplos Prácticos

### Ejemplo 1: Rastrear una Expresión Simple

**Entrada:**
```rust
unsafe {
    *ptr
}
```

**Paso a paso:**

```
1. parse_file() → AST completo
2. PatternDetector detecta "deref_expr" en línea X, columna Y
3. extract_dynamic_elements() es llamado:
   - Busca UnOp::Deref
   - Encuentra ExprPath "ptr"
   - Inserta HashMap { "var": "ptr" }
4. TemplateManager busca template "deref_expr"
   - Encuentra: "Box::new({var})"
5. Reemplaza {var} con "ptr":
   - Resultado: "Box::new(ptr)"
6. syn::parse_str convierte a Expr
7. Reemplaza el unsafe block en el AST
8. prettyplease::unparse() formatea el código
```

### Ejemplo 2: Expresión Compleja

**Entrada:**
```rust
unsafe {
    let x = ptr.offset(5) as i32;
    *ptr = x * 2;
}
```

**¿Qué extrae?**

```rust
// Con syn actual:
// Detecta: "assign_to_deref"
// Extrae: {
//     "var": "ptr",
//     "expr": "x * 2"
// }

// Con regex mejorado:
// Extrae: {
//     "variables": ["x", "ptr", "x"],
//     "numbers": ["5", "2"],
//     "operators": ["*"],
//     "functions": ["offset"]
// }
```

### Ejemplo 3: Patrón No Soportado

**Entrada:**
```rust
unsafe {
    transmute::<T, U>(value)
}
```

**Problema:**
- Patrón `"transmute"` no existe
- Falla silenciosamente en `extract_dynamic_elements`
- El bloque unsafe permanece intacto

**Solución con Regex:**

```rust
let re_function_calls = Regex::new(r"(\w+(?:::\w+)*)\s*<[^>]+>\s*\(").unwrap();
if re_function_calls.is_match(&code_str) {
    // Detecta llamadas a función genéricas
    // Extrae: transmute, Vec, HashMap, etc.
}
```

---

## Casos de Uso Avanzados

### Caso 1: Múltiples Dereferences

```rust
unsafe {
    **ptr  // Doble dereference
}
```

**AST:**
```
Unary(Deref) {
    expr: Unary(Deref) {
        expr: Path("ptr")
    }
}
```

**Mejora con regex:**
```rust
let deref_count = code.matches('*').count();
// Permite manejar casos anidados automáticamente
```

### Caso 2: Operaciones Encadenadas

```rust
unsafe {
    (*ptr).field.method()
}
```

**Extracción con regex:**
```rust
let chain_re = Regex::new(r"(\w+)(?:\.(\w+))+").unwrap();
// Captura cadenas de acceso: ptr.field.method
```

---

## Resumen de Mejoras Propuestas

| Mejora | Beneficio | Complejidad |
|--------|-----------|-------------|
| **Agregar fallback regex** | Maneja patrones no previstos | Baja |
| **Clasificación automática** | Detecta patrones nuevos | Media |
| **Extracción de componentes** | Obtiene variables, funciones, operadores | Media |
| **Análisis recursivo** | Maneja expresiones anidadas | Alta |
| **Caché de patrones** | Mejora rendimiento | Baja |

---

## Referencias Útiles

- **Documentación `syn`**: https://docs.rs/syn/latest/syn/
- **Documentación `regex`**: https://docs.rs/regex/latest/regex/
- **Rust Book - Expresiones**: https://doc.rust-lang.org/book/ch03-03-how-functions-work.html
- **`proc_macro2` y `quote!`**: Para trabajar con tokens y generación de código
