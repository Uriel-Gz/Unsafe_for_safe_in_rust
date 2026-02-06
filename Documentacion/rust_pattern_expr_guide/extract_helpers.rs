// Archivo: extract_helpers.rs
// Módulo de ayuda para mejorar extract_dynamic_elements
// Se puede integrar en el proyecto principal

use std::collections::HashMap;
use regex::Regex;
use syn::{Expr, ExprPath, ExprUnary, UnOp};
use proc_macro2::TokenStream;
use quote::ToTokens;

/// ============================================================================
/// NUEVAS HERRAMIENTAS DE EXTRACCIÓN
/// ============================================================================

/// Estrategia 1: Extracción de Identificadores Únicos
/// Extrae todos los identificadores (variables) del código con regex
pub fn extract_identifiers_with_regex(code: &str) -> Vec<String> {
    let re = Regex::new(r"\b([a-zA-Z_][a-zA-Z0-9_]*)\b").unwrap();
    
    let mut identifiers: Vec<String> = re.captures_iter(code)
        .map(|cap| cap[1].to_string())
        .collect();
    
    // Elimina palabras clave de Rust
    let keywords = vec![
        "unsafe", "let", "const", "static", "mut", "fn", "pub", "crate",
        "impl", "trait", "struct", "enum", "union", "type", "if", "else",
        "match", "loop", "while", "for", "break", "continue", "return",
        "as", "use", "mod", "extern", "where", "self", "Self",
    ];
    
    identifiers.retain(|id| !keywords.contains(&id.as_str()));
    
    // Elimina duplicados manteniendo el orden
    identifiers.sort();
    identifiers.dedup();
    
    identifiers
}

/// Estrategia 2: Clasificación de Tipos de Elementos
pub enum ElementType {
    Variable,
    Function,
    Type,
    Number,
    Operator,
    Keyword,
}

pub struct ClassifiedElement {
    pub name: String,
    pub element_type: ElementType,
}

/// Clasifica elementos en el código usando patrones regex
pub fn classify_elements(code: &str) -> Vec<ClassifiedElement> {
    let mut classified = Vec::new();
    
    // Números
    let num_re = Regex::new(r"\b(\d+)\b").unwrap();
    for cap in num_re.captures_iter(code) {
        classified.push(ClassifiedElement {
            name: cap[1].to_string(),
            element_type: ElementType::Number,
        });
    }
    
    // Funciones: identificador + paréntesis
    let func_re = Regex::new(r"\b([a-zA-Z_][a-zA-Z0-9_]*)\s*\(").unwrap();
    for cap in func_re.captures_iter(code) {
        classified.push(ClassifiedElement {
            name: cap[1].to_string(),
            element_type: ElementType::Function,
        });
    }
    
    // Tipos: después de ::, comienzan con mayúscula
    let type_re = Regex::new(r"::([A-Z][a-zA-Z0-9_]*)").unwrap();
    for cap in type_re.captures_iter(code) {
        classified.push(ClassifiedElement {
            name: cap[1].to_string(),
            element_type: ElementType::Type,
        });
    }
    
    // Operadores
    let op_re = Regex::new(r"([+\-*/%&|^<>=!]+)").unwrap();
    for cap in op_re.captures_iter(code) {
        classified.push(ClassifiedElement {
            name: cap[1].to_string(),
            element_type: ElementType::Operator,
        });
    }
    
    // Variables: identificadores que no son funciones
    let var_re = Regex::new(r"\b([a-zA-Z_][a-zA-Z0-9_]*)\b(?!\s*\()").unwrap();
    for cap in var_re.captures_iter(code) {
        let name = &cap[1];
        if !classified.iter().any(|e| e.name == *name) {
            classified.push(ClassifiedElement {
                name: name.to_string(),
                element_type: ElementType::Variable,
            });
        }
    }
    
    classified
}

/// ============================================================================
/// HERRAMIENTAS DE ANÁLISIS RECURSIVO
/// ============================================================================

/// Extrae recursivamente todas las variables de una expresión Rust
pub fn extract_all_variables_recursive(expr: &Expr) -> Vec<String> {
    let mut vars = Vec::new();
    extract_variables_from_expr(expr, &mut vars);
    
    // Elimina duplicados
    vars.sort();
    vars.dedup();
    
    vars
}

fn extract_variables_from_expr(expr: &Expr, vars: &mut Vec<String>) {
    match expr {
        Expr::Path(path) => {
            if let Some(ident) = path.path.get_ident() {
                vars.push(ident.to_string());
            }
        }
        Expr::Unary(unary) => {
            extract_variables_from_expr(&unary.expr, vars);
        }
        Expr::Binary(binary) => {
            extract_variables_from_expr(&binary.left, vars);
            extract_variables_from_expr(&binary.right, vars);
        }
        Expr::Call(call) => {
            extract_variables_from_expr(&call.func, vars);
            for arg in &call.args {
                extract_variables_from_expr(arg, vars);
            }
        }
        Expr::MethodCall(method) => {
            extract_variables_from_expr(&method.receiver, vars);
            for arg in &method.args {
                extract_variables_from_expr(arg, vars);
            }
        }
        Expr::Field(field) => {
            extract_variables_from_expr(&field.base, vars);
        }
        Expr::Index(index) => {
            extract_variables_from_expr(&index.expr, vars);
            extract_variables_from_expr(&index.index, vars);
        }
        Expr::Reference(refer) => {
            extract_variables_from_expr(&refer.expr, vars);
        }
        Expr::Assign(assign) => {
            extract_variables_from_expr(&assign.left, vars);
            extract_variables_from_expr(&assign.right, vars);
        }
        _ => {
            // Para otros tipos, intenta iterar si es posible
        }
    }
}

/// ============================================================================
/// DETECCIÓN INTELIGENTE DE PATRONES
/// ============================================================================

/// Detecta automáticamente el tipo de patrón sin necesidad de pre-configuración
pub fn auto_detect_pattern(code_str: &str) -> String {
    // Dereference doble
    if Regex::new(r"^\*\*").unwrap().is_match(code_str) {
        return "double_deref".to_string();
    }
    
    // Asignación a puntero
    if Regex::new(r"^\*\w+\s*=").unwrap().is_match(code_str) {
        return "assign_to_deref".to_string();
    }
    
    // Dereference simple
    if Regex::new(r"^\*\w+$").unwrap().is_match(code_str) {
        return "deref_expr".to_string();
    }
    
    // Dirección bruta
    if Regex::new(r"^&raw\s+(const|mut)").unwrap().is_match(code_str) {
        return "raw_addr_expr".to_string();
    }
    
    // Indexación
    if Regex::new(r"^\w+\[").unwrap().is_match(code_str) {
        return "array_index_expr".to_string();
    }
    
    // Referencia mutable
    if Regex::new(r"^&mut\s+\w+$").unwrap().is_match(code_str) {
        return "mutable_ref_expr".to_string();
    }
    
    // Operación aritmética
    if Regex::new(r"[\+\-\*/%]").unwrap().is_match(code_str) {
        return "binary_arith_expr".to_string();
    }
    
    // Transmute
    if Regex::new(r"transmute").unwrap().is_match(code_str) {
        return "transmute_expr".to_string();
    }
    
    // Cast
    if Regex::new(r"\bas\s+").unwrap().is_match(code_str) {
        return "cast_expr".to_string();
    }
    
    "unknown".to_string()
}

/// ============================================================================
/// HERRAMIENTAS DE VALIDACIÓN
/// ============================================================================

/// Valida si un patrón detectado es realmente válido
pub fn validate_pattern(code_str: &str, pattern_kind: &str) -> bool {
    match pattern_kind {
        "deref_expr" => {
            // Debe ser una expresión de dereference válida
            Regex::new(r"^\*\w+$").unwrap().is_match(code_str)
        }
        "assign_to_deref" => {
            // Debe tener forma: *var = expr
            Regex::new(r"^\*\w+\s*=\s*.+$").unwrap().is_match(code_str)
        }
        "raw_addr_expr" => {
            Regex::new(r"^&raw\s+(const|mut)\s+\w+$").unwrap().is_match(code_str)
        }
        "array_index_expr" => {
            Regex::new(r"^\w+\[\d+\]$").unwrap().is_match(code_str)
        }
        "transmute_expr" => {
            Regex::new(r"transmute\s*::<").unwrap().is_match(code_str)
        }
        _ => true, // Desconocido, pero válido
    }
}

/// ============================================================================
/// EXTRACCIÓN INTELIGENTE MEJORADA
/// ============================================================================

/// Versión mejorada que combina syn + regex
pub fn extract_dynamic_elements_v2(
    block_stmts: &[syn::Stmt],
    pattern_kind: &str,
) -> HashMap<String, String> {
    let mut elements = HashMap::new();
    
    // Convierte el bloque a string para análisis con regex
    let code_str = block_stmts.iter()
        .map(|s| format!("{}", quote::quote! { #s }))
        .collect::<Vec<_>>()
        .join("; ");
    
    match pattern_kind {
        "deref_expr" => {
            // Extrae la variable del dereference
            if let Some(capture) = Regex::new(r"\*([a-zA-Z_]\w*)")
                .unwrap()
                .captures(&code_str)
            {
                let var_name = capture[1].to_string();
                elements.insert("var".to_string(), var_name);
            }
        }
        "assign_to_deref" => {
            // Extrae variable y valor
            if let Some(capture) = Regex::new(r"\*([a-zA-Z_]\w*)\s*=\s*(.+)$")
                .unwrap()
                .captures(&code_str)
            {
                elements.insert("var".to_string(), capture[1].to_string());
                elements.insert("expr".to_string(), capture[2].trim().to_string());
            }
        }
        "raw_addr_expr" => {
            if let Some(capture) = Regex::new(r"&raw\s+(?:const|mut)\s+([a-zA-Z_]\w*)")
                .unwrap()
                .captures(&code_str)
            {
                elements.insert("var".to_string(), capture[1].to_string());
            }
        }
        _ => {
            // Extracción genérica
            let variables = extract_identifiers_with_regex(&code_str);
            if !variables.is_empty() {
                elements.insert("variables".to_string(), variables.join(", "));
            }
            
            let numbers = Regex::new(r"\b(\d+)\b")
                .unwrap()
                .captures_iter(&code_str)
                .map(|c| c[1].to_string())
                .collect::<Vec<_>>();
            
            if !numbers.is_empty() {
                elements.insert("numbers".to_string(), numbers.join(", "));
            }
            
            // Expresión completa
            elements.insert("expr".to_string(), code_str);
        }
    }
    
    elements
}

/// ============================================================================
/// DIAGNÓSTICO Y DEBUGGING
/// ============================================================================

/// Imprime un informe detallado sobre un patrón detectado
pub fn debug_pattern_detection(code_str: &str) {
    println!("\n=== DEBUG: Pattern Detection ===");
    println!("Code: {}", code_str);
    
    let detected = auto_detect_pattern(code_str);
    println!("Detected pattern: {}", detected);
    
    let is_valid = validate_pattern(code_str, &detected);
    println!("Pattern valid: {}", is_valid);
    
    let elements = extract_dynamic_elements_v2(&[], &detected);
    println!("Extracted elements: {:?}", elements);
    
    let identifiers = extract_identifiers_with_regex(code_str);
    println!("Identifiers found: {:?}", identifiers);
    
    let classified = classify_elements(code_str);
    println!("Classified elements:");
    for elem in classified {
        println!("  - {}: {:?}", elem.name, elem.element_type);
    }
}

/// ============================================================================
/// PRUEBAS DE EJEMPLO
/// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extract_identifiers() {
        let code = "*ptr + offset";
        let vars = extract_identifiers_with_regex(code);
        assert_eq!(vars, vec!["offset", "ptr"]);
    }
    
    #[test]
    fn test_auto_detect_deref() {
        let code = "*ptr";
        let pattern = auto_detect_pattern(code);
        assert_eq!(pattern, "deref_expr");
    }
    
    #[test]
    fn test_auto_detect_assign() {
        let code = "*ptr = 42";
        let pattern = auto_detect_pattern(code);
        assert_eq!(pattern, "assign_to_deref");
    }
    
    #[test]
    fn test_validate_pattern() {
        assert!(validate_pattern("*ptr", "deref_expr"));
        assert!(validate_pattern("*ptr = 42", "assign_to_deref"));
        assert!(!validate_pattern("invalid", "deref_expr"));
    }
    
    #[test]
    fn test_classify_elements() {
        let code = "ptr + 42";
        let classified = classify_elements(code);
        
        let has_var = classified.iter().any(|e| e.name == "ptr");
        let has_num = classified.iter().any(|e| e.name == "42");
        let has_op = classified.iter().any(|e| e.name == "+");
        
        assert!(has_var);
        assert!(has_num);
        assert!(has_op);
    }
}
