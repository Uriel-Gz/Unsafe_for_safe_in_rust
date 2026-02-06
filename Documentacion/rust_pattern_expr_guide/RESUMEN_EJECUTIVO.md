# 🎓 Resumen Ejecutivo: Extract Dynamic Elements

## En Una Página

### ¿QUÉ HACE extract_dynamic_elements?

```
ENTRADA: Un bloque unsafe y su tipo de patrón
         unsafe { *ptr }  +  "deref_expr"
                                    ↓
        ┌────────────────────────────────────────┐
        │   extract_dynamic_elements(            │
        │       expr_unsafe,                     │
        │       "deref_expr"                     │
        │   )                                    │
        └────────────────────────────────────────┘
                         ↓
SALIDA: Un mapa de placeholders → valores
        HashMap { "var": "ptr" }
```

### LOS 4 PATRONES PRINCIPALES

| Patrón | Entrada | Extrae | Template |
|--------|---------|--------|----------|
| **deref_expr** | `*ptr` | `{var}=ptr` | `Box::new({var})` |
| **assign_to_deref** | `*ptr=42` | `{var}=ptr, {expr}=42` | `mem::replace({var},{expr})` |
| **raw_addr_expr** | `&raw const x` | `{var}=x` | `safe_raw_addr({var})` |
| **otros** | cualquiera | `{expr}=todo` | depende |

### ANATOMÍA DE UNA EXTRACCIÓN

```rust
fn extract_dynamic_elements(expr_unsafe: &ExprUnsafe, pattern_kind: &str) 
    -> HashMap<String, String>
{
    // 1. Obtén el bloque
    let block = &expr_unsafe.block;
    
    // 2. Crea un HashMap vacío
    let mut elements = HashMap::new();
    
    // 3. MATCH sobre el patrón
    match pattern_kind {
        "deref_expr" => {
            // 4. ITERA sobre sentencias
            for stmt in &block.stmts {
                // 5. PATTERN MATCH: ¿es *var?
                if let syn::Stmt::Expr(
                    Expr::Unary(ExprUnary { 
                        op: UnOp::Deref(_),  ← ¿Es un *?
                        expr, 
                        .. 
                    }), 
                    _
                ) = stmt {
                    
                    // 6. EXTRAE: ¿cuál es la variable?
                    if let Expr::Path(ExprPath { path, .. }) = &**expr {
                        if let Some(ident) = path.get_ident() {
                            // 7. INSERTA en el mapa
                            elements.insert("var", ident.to_string());
                        }
                    }
                }
            }
        }
        // ... otros patrones ...
    }
    
    // 8. RETORNA el mapa completo
    elements
}
```

### CÓMO SE USA

```rust
// En modifier.rs, línea 309:

// Paso 1: Detecta patrón
if let Some(pattern) = self.find_matching_pattern(expr_unsafe) {
    
    // Paso 2: Extrae elementos
    let elements = extract_dynamic_elements(expr_unsafe, &pattern.kind);
    // elements = {"var": "ptr"}
    
    // Paso 3: Obtiene template
    if let Some(template) = self.templates.get_template(&pattern.kind) {
        // template = "Box::new({var})"
        
        // Paso 4: Reemplaza placeholders
        let mut replacement = template.clone();
        for (key, value) in &elements {
            // "Box::new({var})" → "Box::new(ptr)"
            replacement = replacement.replace(&format!("{{{}}}", key), value);
        }
        
        // Paso 5: Convierte a código Rust
        if let Ok(new_expr) = syn::parse_str::<Expr>(&replacement) {
            *node = new_expr;  // Reemplaza en el AST
        }
    }
}
```

---

## El Problema Actual

### ✅ Funciona Bien
- Patrones pre-configurados (deref, assign, raw_addr)
- Extrae variables simples (path directo)
- Manejo de sentencias únicas

### ❌ Limitaciones
- No detecta patrones nuevos automáticamente
- No extrae información compleja (anidados, recursivos)
- No maneja bien múltiples sentencias
- Silenciosamente falla si patrón no se reconoce

### Ejemplo que FALLA

```rust
unsafe {
    let offset = 5;
    let new_ptr = ptr.offset(offset);  // ← MethodCall, no Path
    *new_ptr                            // ← Deref de new_ptr
}
```

**¿Qué pasa?**
- Detecta `*new_ptr` (la última sentencia)
- Extrae `{"var": "new_ptr"}` ✓
- **Pierde** que `new_ptr` viene de `ptr.offset(5)` ✗

---

## La Solución: Regex como Fallback

### Mejora 1: Fallback Automático

```rust
// Si syn no encuentra el patrón específico...
_ => {
    // ... usa regex como respaldo
    let var_regex = Regex::new(r"\b([a-zA-Z_]\w*)\b").unwrap();
    let vars: Vec<String> = var_regex.captures_iter(&code_str)
        .map(|c| c[1].to_string())
        .collect();
    
    elements.insert("variables", vars.join(", "));
    elements.insert("expr", code_str);
}
```

**Ventaja:** Nunca falla completamente

### Mejora 2: Detección Automática

```rust
fn auto_detect_pattern(code: &str) -> String {
    if Regex::new(r"^\*\w+\s*=\s*").unwrap().is_match(code) {
        "assign_to_deref"
    } else if Regex::new(r"^&raw\s+").unwrap().is_match(code) {
        "raw_addr_expr"
    } else if Regex::new(r"transmute").unwrap().is_match(code) {
        "transmute_expr"
    } else {
        "unknown"
    }
}
```

**Ventaja:** Descubre nuevos patrones dinámicamente

### Mejora 3: Extracción Recursiva

```rust
fn extract_recursive(expr: &Expr, vars: &mut Vec<String>) {
    match expr {
        Expr::Path(p) => vars.push(p.path.to_string()),
        Expr::Unary(u) => extract_recursive(&u.expr, vars),
        Expr::MethodCall(m) => {
            extract_recursive(&m.receiver, vars);
            for arg in &m.args {
                extract_recursive(arg, vars);
            }
        }
        // ... etc ...
    }
}
```

**Ventaja:** Extrae TODO, incluso anidado

---

## La Anatomía del AST

### Código → AST

```
Código:          unsafe { *ptr }
                         │
                         ↓ syn::parse_file()
                         
AST:             ExprUnsafe
                 └── block: Block
                     └── stmts: Vec[1]
                         └── Stmt::Expr(
                             Expr::Unary {
                                 op: Deref,
                                 expr: Expr::Path { 
                                     path: Identifier("ptr") 
                                 }
                             }
                         )
```

### Pattern Matching en Detalle

```rust
// Nivel 1: ¿Es una sentencia de expresión?
if let syn::Stmt::Expr(...) { 
    // Nivel 2: ¿Es una expresión unaria?
    if let Expr::Unary(...) { 
        // Nivel 3: ¿Es un dereference?
        if matches!(op, UnOp::Deref(_)) { 
            // Nivel 4: ¿Es una variable?
            if let Expr::Path(p) = &expr { 
                // ✓ Encontrado!
            } 
        } 
    } 
}
```

---

## Regex Útiles

```regex
Variables:      \b([a-zA-Z_]\w*)\b
Números:        \b(\d+)\b
Operadores:     ([+\-*/%&|^<>=!]+)
Funciones:      \b(\w+)\s*\(
Tipos:          ::([A-Z]\w*)
Dereference:    \*([a-zA-Z_]\w*)
Asignación:     \*(\w+)\s*=\s*(.+)
Raw Address:    &raw\s+(const|mut)\s+(\w+)
Transmute:      transmute\s*::<
```

---

## Cheat Sheet: Casos Límite

### Caso 1: Doble Dereference (**ptr)

```rust
// ¿Funciona?
Expr::Unary {
    op: Deref,
    expr: Expr::Unary {  ← ¡Otro Unary!
        op: Deref,
        expr: Path("ptr")
    }
}
// ✓ SÍ, porque expr puede ser cualquier Expr

// ¿Qué extrae?
// **ptr → {"var": "ptr"} pero pierde que es **
// Mejora: Contar dereferencias
```

### Caso 2: Method Call (ptr.offset(5))

```rust
// ¿Funciona?
if let Expr::Path(...) = expr { }  // NO COINCIDE
// Porque es Expr::MethodCall, no Path

// Mejora: Búsqueda recursiva
fn find_base_var(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Path(p) => Some(...),
        Expr::MethodCall(m) => find_base_var(&m.receiver),  ← Recurse
        _ => None,
    }
}
```

### Caso 3: Patrón Desconocido (transmute)

```rust
// Actual: Entra en match _ → extrae expr genérico
// ¿Pierde información?
// Sí, no sabe que es transmute

// Mejora: Detecta transmute
if code.contains("transmute") {
    return "transmute_expr";
}
// Luego crea template específico para transmute
```

---

## Roadmap de 3 Fases

### Fase 1: ⭐ AHORA (1-2 horas)
```rust
// En extract_dynamic_elements, reemplaza:
_ => {
    // Viejo: retorna vacío
    if !block.stmts.is_empty() { ... }
}

// Por:
_ => {
    // Nuevo: usa regex + recursión
    let vars = extract_all_variables_recursive(...);
    elements.insert("variables", vars.join(", "));
}
```

### Fase 2: 📅 PRÓXIMO (4-6 horas)
```rust
// Agrega auto_detect_pattern()
// Intégra en PatternDetector
// Agrega tests para nuevos patrones
```

### Fase 3: 🚀 FUTURO (1-2 días)
```rust
// Caché de patrones (compilar regex 1 sola vez)
// Paralelización con rayon
// Análisis de tipos con ariadne
```

---

## Verificación: ¿Lo Entendiste?

Responde sin mirar arriba:

1. ¿Qué retorna `extract_dynamic_elements`?
   - [ ] Una lista de variables
   - [ ] Un HashMap de placeholders → valores ✓
   - [ ] El código reemplazado

2. ¿Cómo sabe qué extraer?
   - [ ] Adivinando
   - [ ] De acuerdo al `pattern_kind` ✓
   - [ ] Leyendo un archivo de configuración

3. ¿Cuál es el regex para variables?
   - [ ] `\w+`
   - [ ] `\b([a-zA-Z_]\w*)\b` ✓
   - [ ] `[a-z]+`

4. ¿Qué hacer si un patrón no se reconoce?
   - [ ] Dejar el unsafe intacto
   - [ ] Usar regex como fallback ✓
   - [ ] Tirar error

5. ¿Cómo extraer de **ptr?
   - [ ] No se puede
   - [ ] Contar dereferencias recursivamente ✓
   - [ ] Ignorar los primeros *

---

## Conclusión

La función `extract_dynamic_elements` es el **corazón** de la transformación de código unsafe. 

- **Hoy:** Extrae información de patrones pre-definidos
- **Mañana:** Debería detectar patrones automáticamente
- **Futuro:** Debería entender la semántica del código

**Tu tarea:** Mejorarlo usando regex + recursión.

**Recursos disponibles:**
1. ✅ [GUIA_EXPRESIONES_RUST.md](GUIA_EXPRESIONES_RUST.md) - Teoría
2. ✅ [EJEMPLOS_EXTRACT_DYNAMIC.md](EJEMPLOS_EXTRACT_DYNAMIC.md) - Ejemplos
3. ✅ [src/extract_helpers.rs](src/extract_helpers.rs) - Código
4. ✅ [CHEAT_SHEET_EXPRESIONES.md](CHEAT_SHEET_EXPRESIONES.md) - Referencia

**Next step:** Abre `GUIA_EXPRESIONES_RUST.md` y comienza. 🚀
