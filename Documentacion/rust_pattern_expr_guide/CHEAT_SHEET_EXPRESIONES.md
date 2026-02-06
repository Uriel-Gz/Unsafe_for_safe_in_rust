# Quick Reference: Expresiones Rust y Extract Dynamic Elements

## Mapa Mental Visual

```
EXPRESIONES RUST
    │
    ├─ TIPOS
    │   ├─ Aritméticas: 5 + 6
    │   ├─ Comparación: a > b
    │   ├─ Unarias: *ptr, !x, -y
    │   ├─ Binarias: a + b, x && y
    │   ├─ Llamadas: func()
    │   ├─ Acceso: arr[0], obj.field
    │   ├─ Control: if x { } else { }
    │   ├─ Bloques: { statements }
    │   └─ Unsafe: unsafe { ... }
    │
    ├─ ESTRUCTURAS (AST)
    │   ├─ Expr (enumeración general)
    │   ├─ ExprUnary (unaria)
    │   ├─ ExprBinary (binaria)
    │   ├─ ExprPath (variable/ruta)
    │   ├─ ExprCall (llamada)
    │   ├─ ExprUnsafe (bloque unsafe)
    │   └─ Block (bloque de código)
    │
    └─ PATRONES
        ├─ deref_expr: *var
        ├─ assign_to_deref: *ptr = value
        ├─ raw_addr_expr: &raw const/mut var
        ├─ binary_arith_expr: a + b
        ├─ array_index_expr: arr[i]
        ├─ mutable_ref_expr: &mut x
        └─ unsafe_block: {...}
```

---

## Cheat Sheet: Patrones Comunes

### 1️⃣ DEREFERENCE SIMPLE

```rust
// Código
unsafe { *ptr }

// Pattern matching en syn
if let Expr::Unary(ExprUnary { op: UnOp::Deref(_), expr, .. }) = expr {
    // expr = Box::new(Expr::Path(...))
}

// Extracción
extract_dynamic_elements() → {"var": "ptr"}

// Template
"Box::new({var})" → "Box::new(ptr)"
```

### 2️⃣ ASIGNACIÓN A PUNTERO

```rust
// Código
unsafe { *ptr = 42 }

// Pattern matching
if let Expr::Assign(assign) = expr {
    // assign.left = *ptr (Unary Deref)
    // assign.right = 42 (Literal)
}

// Extracción
{"var": "ptr", "expr": "42"}

// Template
"mem::replace({var}, {expr})" → "mem::replace(ptr, 42)"
```

### 3️⃣ DIRECCIÓN BRUTA

```rust
// Código
unsafe { &raw const x }

// Pattern matching
if let Expr::RawAddr(raw) = expr {
    // raw.expr = Expr::Path(x)
    // raw.mutability = None (const)
}

// Extracción
{"var": "x"}
```

### 4️⃣ LLAMADA A FUNCIÓN

```rust
// Código
unsafe { transmute::<T, U>(value) }

// Pattern matching
if let Expr::Call(call) = expr {
    // call.func = Path to transmute
    // call.args = [value]
}

// Regex
r"(\w+(?:::\w+)*)\s*<[^>]+>\s*\("
// Captura: transmute
```

---

## Flujo de Procesamiento

```
┌─────────────────────────────────────────────────────────────┐
│ 1. LECTURA                                                  │
│    fs::read_to_string(archivo.rs)                           │
│    ↓                                                         │
│ 2. PARSEO                                                   │
│    syn::parse_file(&source_code)                            │
│    ↓ Resultado: File { items: [...] }                       │
│ 3. DETECCIÓN DE PATRONES                                    │
│    PatternDetector::visit_file(&ast)                        │
│    ↓ Resultado: Vec<PatternInfo>                            │
│ 4. EXTRACCIÓN DE ELEMENTOS                                  │
│    extract_dynamic_elements(expr, pattern_kind)             │
│    ↓ Resultado: HashMap<String, String>                     │
│ 5. BÚSQUEDA DE TEMPLATE                                     │
│    templates.get_template(pattern_kind)                     │
│    ↓ Resultado: Some("Box::new({var})")                     │
│ 6. REEMPLAZO                                                │
│    "Box::new({var})".replace("{var}", "ptr")               │
│    ↓ Resultado: "Box::new(ptr)"                             │
│ 7. PARSEO DEL REEMPLAZO                                     │
│    syn::parse_str::<Expr>(&replacement)                     │
│    ↓ Resultado: Expr (AST del nuevo código)                │
│ 8. APLICACIÓN AL AST                                        │
│    *node = new_expr (VisitMut)                              │
│    ↓ Modifica el árbol                                      │
│ 9. ESCRITURA                                                │
│    prettyplease::unparse(&ast)                              │
│    fs::write(&output_file, formatted_code)                  │
│    ↓ Archivo transformado                                   │
└─────────────────────────────────────────────────────────────┘
```

---

## Lookup: ¿Qué Usar Cuándo?

| Necesito... | Usar... | Ejemplo |
|-------------|---------|---------|
| Recorrer el AST | `Visit` / `VisitMut` | `impl VisitMut for Modifier` |
| Detectar bloques unsafe | `visit_expr_mut` | `if let Expr::Unsafe` |
| Extraer variable de *ptr | Pattern matching syn | `if let Expr::Path(p) = &expr` |
| Buscar identificadores | Regex | `\b([a-zA-Z_]\w*)\b` |
| Buscar números | Regex | `\b(\d+)\b` |
| Convertir AST a string | `to_token_stream()` | `expr.to_token_stream().to_string()` |
| Crear código nuevo | `syn::parse_str` | `syn::parse_str::<Expr>("Box::new(ptr)")` |
| Formatar código | `prettyplease::unparse` | `prettyplease::unparse(&ast)` |
| Validar estructura | `validate_morphology` | Checar que sea *var no **var |

---

## Errores Comunes y Soluciones

### ❌ Error 1: "No coincide el patrón"

```rust
// PROBLEMA
let code = unsafe { **ptr }; // Doble dereference

// El pattern "deref_expr" solo busca *var, no **var
if let syn::Stmt::Expr(Expr::Unary(
    ExprUnary { op: UnOp::Deref(_), expr, .. }
), _) = stmt {
    // expr = Expr::Unary (otro Deref)
    // ✓ SÍ FUNCIONA porque expr puede ser Unary
}

// SOLUCIÓN: Contar niveles de dereference
fn count_dereferences(expr: &Expr) -> usize {
    match expr {
        Expr::Unary(u) if matches!(u.op, UnOp::Deref(_)) => {
            1 + count_dereferences(&u.expr)
        }
        _ => 0,
    }
}

let level = count_dereferences(expr);
// level = 2 para **ptr
```

### ❌ Error 2: "No extrae el variable"

```rust
// PROBLEMA
unsafe { ptr.offset(5) }

// El pattern "deref_expr" busca Expr::Path directo
// Pero aquí es Expr::MethodCall
if let Expr::Path(p) = expr { } // NO COINCIDE

// SOLUCIÓN: Manejo recursivo
fn extract_base_var(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Path(p) => p.path.get_ident().map(|i| i.to_string()),
        Expr::MethodCall(m) => extract_base_var(&m.receiver),
        Expr::Call(c) => extract_base_var(&c.func),
        Expr::Field(f) => extract_base_var(&f.base),
        _ => None,
    }
}

let var = extract_base_var(expr); // Some("ptr")
```

### ❌ Error 3: "El bloque tiene múltiples sentencias"

```rust
// PROBLEMA
unsafe {
    let x = 5;
    let y = x + 1;
    *ptr = y;
}

// El pattern "assign_to_deref" busca en stmts[0]
// Pero la asignación está en stmts[2]

// SOLUCIÓN: Iterar sobre todas las sentencias
for stmt in &block.stmts {
    if let syn::Stmt::Expr(Expr::Assign(assign), _) = stmt {
        // Procesa TODAS las asignaciones
    }
}
```

### ❌ Error 4: "No reconoce el patrón"

```rust
// PROBLEMA
unsafe { transmute::<T, U>(x) }

// No existe patrón "transmute"
// extract_dynamic_elements entra en el match por defecto

// SOLUCIÓN: Detección automática
let pattern = auto_detect_pattern(&code_string);
// "transmute_expr"

// O usar regex como fallback
let code_str = format!("{}", quote! { ... });
if let Some(cap) = Regex::new(r"transmute").unwrap().captures(&code_str) {
    // Maneja transmute
}
```

---

## Optimizaciones Propuestas

### Optimización 1: Caché de Patrones

```rust
use std::sync::Arc;
use parking_lot::RwLock;

lazy_static::lazy_static! {
    static ref PATTERN_CACHE: RwLock<HashMap<String, PatternInfo>> = 
        RwLock::new(HashMap::new());
}

fn find_pattern_cached(code: &str) -> Option<PatternInfo> {
    // Busca en caché primero
    if let Some(pattern) = PATTERN_CACHE.read().get(code) {
        return Some(pattern.clone());
    }
    
    // Si no está, detecta
    let pattern = detect_pattern(code)?;
    
    // Guarda en caché
    PATTERN_CACHE.write().insert(code.to_string(), pattern.clone());
    
    Some(pattern)
}
```

### Optimización 2: Paralelización

```rust
use rayon::prelude::*;

pub fn process_files_parallel(
    files: Vec<PathBuf>,
    templates: &TemplateManager,
) -> Vec<Result<String>> {
    files.par_iter()
        .map(|file| process_single_file(file, templates))
        .collect()
}
```

### Optimización 3: Compilación Regex

```rust
use once_cell::sync::Lazy;

static DEREF_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^\*([a-zA-Z_]\w*)$").unwrap()
});

static ASSIGN_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^\*([a-zA-Z_]\w*)\s*=\s*(.+)$").unwrap()
});

// Las regex se compilan UNA SOLA VEZ
fn extract_fast(code: &str) -> Option<String> {
    DEREF_REGEX.captures(code)
        .map(|c| c[1].to_string())
}
```

---

## Resumen de Funciones Clave

```rust
// MAIN
replace_unsafe_code(input_dir, output_dir, patterns_dir)
├── parse_file(source_code) → File
├── PatternDetector::visit_file() → Vec<PatternInfo>
├── validate_morphology() → bool
├── extract_dynamic_elements() → HashMap
├── TemplateManager::get_template() → Option<String>
├── syn::parse_str(replacement) → Expr
└── prettyplease::unparse(ast) → String

// HELPERS (en extract_helpers.rs)
extract_identifiers_with_regex(code) → Vec<String>
classify_elements(code) → Vec<ClassifiedElement>
extract_all_variables_recursive(expr) → Vec<String>
auto_detect_pattern(code) → String
validate_pattern(code, pattern) → bool
```

---

## Roadmap de Mejoras

### Fase 1: Corto Plazo (Bajo Esfuerzo)
- ✅ Agregar fallback regex en `extract_dynamic_elements`
- ✅ Mejorar logging/debugging
- ✅ Manejo de casos anidados (**)

### Fase 2: Mediano Plazo (Esfuerzo Medio)
- 📋 Auto-detección de patrones
- 📋 Extracción recursiva
- 📋 Caché de patrones
- 📋 Manejo de múltiples templates

### Fase 3: Largo Plazo (Alto Esfuerzo)
- 📋 Análisis de tipos
- 📋 Generación de templates automáticos
- 📋 Machine learning para patrones nuevos
- 📋 Validación de seguridad post-transformación

---

## Links Útiles

### Documentación
- `syn` AST: https://docs.rs/syn/latest/syn/enum.Expr.html
- `regex` patterns: https://docs.rs/regex/latest/regex/
- `quote!` macro: https://docs.rs/quote/latest/quote/

### Herramientas
- AST Viewer: https://ast.rs/ (visualiza ASTs de Rust)
- Regex Tester: https://regex101.com/
- Rust Playground: https://play.rust-lang.org/

### Blogs/Tutoriales
- "Creating a Rust function procedural macro": https://docs.rs/proc-macro/
- "Understanding Rust's Module System": https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-modules-and-paths.html
