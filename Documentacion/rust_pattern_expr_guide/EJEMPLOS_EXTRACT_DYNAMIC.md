# Ejemplos Prácticos: Extract Dynamic Elements

## Tabla de Contenidos
1. [Ejemplos Paso a Paso](#ejemplos-paso-a-paso)
2. [Implementación Mejorada](#implementación-mejorada)
3. [Casos de Prueba](#casos-de-prueba)
4. [Debugging Guide](#debugging-guide)

---

## Ejemplos Paso a Paso

### Ejemplo 1: Deref Simple (`*ptr`)

#### Código Original
```rust
unsafe {
    *ptr
}
```

#### Visualización del AST
```
ExprUnsafe
├── unsafe_token: unsafe
└── block: Block
    └── stmts: Vec[1]
        └── Stmt::Expr(
            Expr::Unary {
                op: UnOp::Deref,      ← Operador *
                expr: *ptr_expr,
                span: ...
            },
            None
        )
            └── ptr_expr = Expr::Path {
                path: Path {
                    segments: [PathSegment { ident: "ptr" }]
                }
            }
```

#### Traza de Ejecución en extract_dynamic_elements

```
pattern_kind = "deref_expr"

block = &expr_unsafe.block
elements = HashMap::new()

for stmt in &block.stmts {
    // stmt = Stmt::Expr(Expr::Unary(...), None)
    
    // PASO 1: Pattern matching en el tipo Stmt
    ✓ if let syn::Stmt::Expr(Expr::Unary(...), _) = stmt
    
    // PASO 2: Desempaquetamos ExprUnary
    ExprUnary { 
        op: UnOp::Deref(_),  ✓ COINCIDE
        expr: ptr_expr,      ← Capturamos esto
        ..
    }
    
    // PASO 3: Verificamos que expr sea un Path
    ✓ if let Expr::Path(ExprPath { path, .. }) = &**expr
    
    // PASO 4: Extraemos el identificador
    path = Path { segments: [...] }
    ✓ if let Some(ident) = path.get_ident()
        ident = Ident { "ptr" }
        
    // PASO 5: Insertamos en HashMap
    elements.insert("var", "ptr")
    println!("Found deref of variable: ptr")
}

// RESULTADO FINAL
elements = HashMap {
    "var": "ptr"
}
```

#### Salida
```
Found deref of variable: ptr
Extracted elements for pattern deref_expr: {"var": "ptr"}
```

#### Uso en Template
```rust
template = "Box::new({var})"
elements = { "var": "ptr" }

// Reemplazo:
replacement = "Box::new({var})".replace("{var}", "ptr")
           = "Box::new(ptr)"

// Resultado final:
unsafe { *ptr } → Box::new(ptr)
```

---

### Ejemplo 2: Asignación a Puntero (`*ptr = value`)

#### Código Original
```rust
unsafe {
    *ptr = 42
}
```

#### Visualización del AST
```
ExprUnsafe
└── block: Block
    └── stmts: Vec[1]
        └── Stmt::Expr(
            Expr::Assign {
                left: *left_expr,
                eq_token: =,
                right: *right_expr
            },
            None
        )
            ├── left_expr = Expr::Unary {
            │   op: UnOp::Deref,
            │   expr: Expr::Path { path: "ptr" }
            │ }
            └── right_expr = Expr::Lit {
                lit: Lit::Int { value: 42 }
            }
```

#### Traza de Ejecución

```
pattern_kind = "assign_to_deref"

for stmt in &block.stmts {
    // PASO 1: Detectamos Assign
    ✓ if let syn::Stmt::Expr(Expr::Assign(assign), _) = stmt
    
    // PASO 2: Verificamos lado izquierdo (*var)
    ✓ if let Expr::Unary(ExprUnary { 
        op: UnOp::Deref(_),
        expr,
        ..
    }) = &*assign.left
    
    // PASO 3: Extraemos variable del puntero
    ✓ if let Expr::Path(ExprPath { path, .. }) = &**expr
    ✓ if let Some(ident) = path.get_ident()
        elements.insert("var", "ptr")
    
    // PASO 4: Extraemos lado derecho (valor)
    let right_tokens = assign.right.to_token_stream()
    // TokenStream { Lit(42) }
    let right_string = right_tokens.to_string()
    // "42"
    elements.insert("expr", "42")
}

// RESULTADO FINAL
elements = HashMap {
    "var": "ptr",
    "expr": "42"
}
```

#### Salida
```
Found deref of variable: ptr
Extracted elements for pattern assign_to_deref: {
    "var": "ptr",
    "expr": "42"
}
```

#### Uso en Template
```rust
template = "mem::replace({var}, {expr})"
elements = {
    "var": "ptr",
    "expr": "42"
}

// Reemplazo:
replacement = "mem::replace({var}, {expr})"
replacement = replacement.replace("{var}", "ptr")
            = "mem::replace(ptr, {expr})"
replacement = replacement.replace("{expr}", "42")
            = "mem::replace(ptr, 42)"

// Resultado final:
unsafe { *ptr = 42 } → mem::replace(ptr, 42)
```

---

### Ejemplo 3: Dirección Bruta (`&raw const x`)

#### Código Original
```rust
unsafe {
    &raw const buffer
}
```

#### Visualización del AST
```
ExprUnsafe
└── block: Block
    └── stmts: Vec[1]
        └── Stmt::Expr(
            Expr::RawAddr {
                and_token: &,
                raw_token: raw,
                mutability: None,     ← const (no mutable)
                expr: *expr
            },
            None
        )
            └── expr = Expr::Path { path: "buffer" }
```

#### Traza de Ejecución

```
pattern_kind = "raw_addr_expr"

for stmt in &block.stmts {
    // PASO 1: Detectamos RawAddr
    ✓ if let syn::Stmt::Expr(Expr::RawAddr(raw_addr), _) = stmt
    
    // PASO 2: Extraemos la expresión entera
    let expr_tokens = raw_addr.expr.to_token_stream()
    // TokenStream { Path(buffer) }
    let expr_string = expr_tokens.to_string()
    // "buffer"
    elements.insert("var", "buffer")
}

// RESULTADO FINAL
elements = HashMap {
    "var": "buffer"
}
```

#### Uso en Template
```rust
template = "safe_raw_addr({var})"
elements = { "var": "buffer" }

// Resultado final:
unsafe { &raw const buffer } → safe_raw_addr(buffer)
```

---

### Ejemplo 4: Caso Complejo (Múltiples Sentencias)

#### Código Original
```rust
unsafe {
    let offset = 5;
    let new_ptr = ptr.offset(offset);
    *new_ptr
}
```

#### Visualización del AST
```
ExprUnsafe
└── block: Block
    └── stmts: Vec[3]
        ├── Stmt::Local {
        │   pat: Pat::Ident { ident: "offset" },
        │   init: Some(Expr::Lit { 5 })
        │ }
        ├── Stmt::Local {
        │   pat: Pat::Ident { ident: "new_ptr" },
        │   init: Some(Expr::Call {
        │       func: Expr::Path { "ptr.offset" },
        │       args: [Expr::Path { "offset" }]
        │   })
        │ }
        └── Stmt::Expr(
            Expr::Unary {
                op: UnOp::Deref,
                expr: Expr::Path { "new_ptr" }
            },
            None
        )
```

#### ¿Qué Sucede en extract_dynamic_elements?

```
pattern_kind = "deref_expr"
block.stmts.len() = 3

for stmt in &block.stmts {
    // stmt[0]: Stmt::Local (no es Stmt::Expr) → SKIP
    // stmt[1]: Stmt::Local (no es Stmt::Expr) → SKIP
    // stmt[2]: Stmt::Expr(Expr::Unary(...))
    
    ✓ if let syn::Stmt::Expr(Expr::Unary(...), _) = stmt[2]
    
    // Detecta: *new_ptr
    ✓ path.get_ident() = "new_ptr"
    elements.insert("var", "new_ptr")
}

// RESULTADO
elements = HashMap {
    "var": "new_ptr"
}
```

#### ¿Problema?

**Este ejemplo muestra una limitación:**

- Solo extrae `new_ptr` del dereference final
- Pierde contexto de `offset` y la inicialización de `new_ptr`
- No entiende que `new_ptr = ptr.offset(5)`

**Solución propuesta** (ver implementación mejorada abajo)

---

## Implementación Mejorada

### Versión 1: Extracción Recursiva

```rust
use std::collections::HashMap;

/// Extrae todos los elementos relevantes recursivamente
fn extract_elements_recursive(expr: &Expr, pattern_kind: &str) -> HashMap<String, String> {
    let mut elements = HashMap::new();
    let mut visited = std::collections::HashSet::new();
    
    fn visit_expr(
        expr: &Expr,
        elements: &mut HashMap<String, String>,
        visited: &mut std::collections::HashSet<String>,
    ) {
        let expr_str = expr.to_token_stream().to_string();
        if visited.contains(&expr_str) {
            return;
        }
        visited.insert(expr_str.clone());
        
        match expr {
            Expr::Path(path) => {
                if let Some(ident) = path.path.get_ident() {
                    elements.insert(format!("var_{}", ident), ident.to_string());
                }
            }
            Expr::Unary(unary) => {
                visit_expr(&unary.expr, elements, visited);
            }
            Expr::Binary(binary) => {
                visit_expr(&binary.left, elements, visited);
                visit_expr(&binary.right, elements, visited);
            }
            Expr::Call(call) => {
                visit_expr(&call.func, elements, visited);
                for arg in &call.args {
                    visit_expr(arg, elements, visited);
                }
            }
            Expr::MethodCall(method) => {
                visit_expr(&method.receiver, elements, visited);
                for arg in &method.args {
                    visit_expr(arg, elements, visited);
                }
            }
            _ => {}
        }
    }
    
    visit_expr(expr, &mut elements, &mut visited);
    elements
}
```

**Ventajas:**
- Extrae recursivamente todas las variables
- Detecta funciones anidadas
- Maneja expresiones complejas

**Ejemplo:**
```rust
// Entrada: ptr.offset(5)
// Salida: {
//     "var_ptr": "ptr",
//     "func_offset": "offset"
// }
```

---

### Versión 2: Con Expresiones Regulares

```rust
use regex::Regex;

/// Extrae elementos usando regex + syn (enfoque híbrido)
fn extract_elements_hybrid(
    expr_unsafe: &ExprUnsafe,
    pattern_kind: &str,
) -> HashMap<String, String> {
    let mut elements = HashMap::new();
    let block = &expr_unsafe.block;
    let code_str = block.stmts.iter()
        .map(|s| quote! { #s }.to_string())
        .collect::<Vec<_>>()
        .join("; ");
    
    // Variables: identificadores válidos en Rust
    let var_regex = Regex::new(r"\b([a-zA-Z_][a-zA-Z0-9_]*)\b").unwrap();
    let vars: Vec<String> = var_regex.captures_iter(&code_str)
        .map(|c| c[1].to_string())
        .collect::<std::collections::HashSet<_>>()  // Elimina duplicados
        .into_iter()
        .collect();
    
    if !vars.is_empty() {
        elements.insert("variables".to_string(), vars.join(", "));
    }
    
    // Números
    let num_regex = Regex::new(r"\b(\d+)\b").unwrap();
    let nums: Vec<String> = num_regex.captures_iter(&code_str)
        .map(|c| c[1].to_string())
        .collect();
    
    if !nums.is_empty() {
        elements.insert("numbers".to_string(), nums.join(", "));
    }
    
    // Operadores
    let op_regex = Regex::new(r"([+\-*/%&|^<>=!]+)").unwrap();
    let ops: Vec<String> = op_regex.captures_iter(&code_str)
        .map(|c| c[1].to_string())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    
    if !ops.is_empty() {
        elements.insert("operators".to_string(), ops.join(", "));
    }
    
    // Llamadas a función: identificador seguido de paréntesis
    let func_regex = Regex::new(r"(\w+)\s*\(").unwrap();
    let funcs: Vec<String> = func_regex.captures_iter(&code_str)
        .map(|c| c[1].to_string())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    
    if !funcs.is_empty() {
        elements.insert("functions".to_string(), funcs.join(", "));
    }
    
    // Tipos: palabras después de ::
    let type_regex = Regex::new(r"::([A-Z]\w*)").unwrap();
    let types: Vec<String> = type_regex.captures_iter(&code_str)
        .map(|c| c[1].to_string())
        .collect();
    
    if !types.is_empty() {
        elements.insert("types".to_string(), types.join(", "));
    }
    
    elements
}
```

**Ejemplo de Uso:**

```rust
unsafe {
    let offset = 5;
    let new_ptr = ptr.offset(offset);
    *new_ptr
}

// Resultado:
HashMap {
    "variables": "offset, new_ptr, ptr",
    "numbers": "5",
    "operators": "*",
    "functions": "offset"
}
```

---

### Versión 3: Extracción Inteligente por Patrón

```rust
fn extract_elements_smart(
    expr_unsafe: &ExprUnsafe,
    pattern_kind: &str,
) -> HashMap<String, String> {
    let mut elements = HashMap::new();
    let block = &expr_unsafe.block;
    
    match pattern_kind {
        "deref_expr" => {
            // *var → extrae "var"
            for stmt in &block.stmts {
                if let syn::Stmt::Expr(Expr::Unary(
                    ExprUnary { 
                        op: UnOp::Deref(_), 
                        expr, 
                        .. 
                    }
                ), _) = stmt {
                    elements.insert(
                        "expr_full".to_string(),
                        expr.to_token_stream().to_string()
                    );
                    
                    // Intenta extraer identificador
                    if let Expr::Path(path) = &**expr {
                        if let Some(ident) = path.path.get_ident() {
                            elements.insert("var".to_string(), ident.to_string());
                        }
                    }
                }
            }
        }
        
        "assign_to_deref" => {
            // *ptr = value → extrae "ptr" y "value"
            for stmt in &block.stmts {
                if let syn::Stmt::Expr(Expr::Assign(assign), _) = stmt {
                    // Lado izquierdo
                    if let Expr::Unary(ExprUnary { 
                        op: UnOp::Deref(_), 
                        expr, 
                        .. 
                    }) = &*assign.left {
                        if let Expr::Path(path) = &**expr {
                            if let Some(ident) = path.path.get_ident() {
                                elements.insert("ptr".to_string(), ident.to_string());
                            }
                        }
                    }
                    
                    // Lado derecho
                    elements.insert(
                        "value".to_string(),
                        assign.right.to_token_stream().to_string()
                    );
                    
                    // Valor completo de la asignación
                    elements.insert(
                        "assign_full".to_string(),
                        format!(
                            "{} = {}",
                            assign.left.to_token_stream().to_string(),
                            assign.right.to_token_stream().to_string()
                        )
                    );
                }
            }
        }
        
        _ => {
            // Fallback: extrae todo
            let full_code = block.stmts.iter()
                .map(|s| s.to_token_stream().to_string())
                .collect::<Vec<_>>()
                .join("; ");
            
            elements.insert("full_code".to_string(), full_code);
        }
    }
    
    elements
}
```

---

## Casos de Prueba

### Test 1: Dereference Simple

```rust
#[test]
fn test_deref_simple() {
    let code = quote! {
        unsafe {
            *ptr
        }
    };
    
    let file = syn::parse2::<File>(code).unwrap();
    // ... extraer ExprUnsafe ...
    
    let elements = extract_dynamic_elements(&expr_unsafe, "deref_expr");
    
    assert_eq!(elements.get("var"), Some(&"ptr".to_string()));
}
```

### Test 2: Asignación Compleja

```rust
#[test]
fn test_assign_complex() {
    let code = quote! {
        unsafe {
            *ptr = calculate(a, b) + offset
        }
    };
    
    let file = syn::parse2::<File>(code).unwrap();
    let elements = extract_dynamic_elements(&expr_unsafe, "assign_to_deref");
    
    assert_eq!(elements.get("var"), Some(&"ptr".to_string()));
    assert!(elements.get("expr").unwrap().contains("calculate"));
    assert!(elements.get("expr").unwrap().contains("offset"));
}
```

### Test 3: Anidamiento

```rust
#[test]
fn test_nested_deref() {
    let code = quote! {
        unsafe {
            **ptr
        }
    };
    
    let elements = extract_dynamic_elements(&expr_unsafe, "deref_expr");
    
    // Debería detectar el doble dereference
    assert!(elements.get("expr").unwrap().contains("*"));
}
```

---

## Debugging Guide

### Técnica 1: Inspeccionar el AST

```rust
fn debug_ast(expr_unsafe: &ExprUnsafe) {
    println!("=== DEBUG AST ===");
    println!("Block statements count: {}", expr_unsafe.block.stmts.len());
    
    for (i, stmt) in expr_unsafe.block.stmts.iter().enumerate() {
        println!("\n[Statement {}]", i);
        match stmt {
            syn::Stmt::Local(local) => {
                println!("  Type: Local");
                println!("  Init: {:?}", local.init.is_some());
            }
            syn::Stmt::Item(item) => {
                println!("  Type: Item");
            }
            syn::Stmt::Expr(expr, semi) => {
                println!("  Type: Expr");
                println!("  Content: {}", expr.to_token_stream());
                println!("  Has semicolon: {}", semi.is_some());
            }
            syn::Stmt::Macro(mac) => {
                println!("  Type: Macro");
            }
        }
    }
}
```

### Técnica 2: Registrar Pattern Matching

```rust
fn extract_with_debug(expr_unsafe: &ExprUnsafe) -> HashMap<String, String> {
    let mut elements = HashMap::new();
    
    for (i, stmt) in expr_unsafe.block.stmts.iter().enumerate() {
        println!("[{}] Checking statement type...", i);
        
        if let syn::Stmt::Expr(expr, _) = stmt {
            println!("    ✓ Is Expr");
            
            if let Expr::Unary(unary) = expr {
                println!("      ✓ Is Unary");
                
                if matches!(unary.op, UnOp::Deref(_)) {
                    println!("        ✓ Is Deref (*)");
                    
                    let result = unary.expr.to_token_stream().to_string();
                    println!("        → Value: {}", result);
                    elements.insert("var".to_string(), result);
                } else {
                    println!("        ✗ Not Deref");
                }
            } else {
                println!("      ✗ Not Unary");
            }
        } else {
            println!("    ✗ Not Expr");
        }
    }
    
    elements
}
```

### Técnica 3: Comparar ASTs

```rust
fn compare_code_structures(code1: &str, code2: &str) {
    let file1 = syn::parse_str::<File>(code1).unwrap();
    let file2 = syn::parse_str::<File>(code2).unwrap();
    
    let ast1 = format!("{:#?}", file1);
    let ast2 = format!("{:#?}", file2);
    
    println!("=== Code 1 AST ===\n{}", ast1);
    println!("=== Code 2 AST ===\n{}", ast2);
    
    // Puedes usar herramientas como `diff` para comparer
}
```

