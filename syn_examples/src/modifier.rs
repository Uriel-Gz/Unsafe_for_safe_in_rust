use anyhow::{Context, Result};
use crate::pattern_detector::{PatternDetector, PatternInfo};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use syn::spanned::Spanned;
use syn::visit::Visit;
use syn::visit_mut::VisitMut;
use syn::{Expr, ExprUnsafe, ExprUnary, ExprPath, File, parse_file, UnOp};
use walkdir::WalkDir;

/// Gestor de templates para reemplazos seguros
struct TemplateManager {
    templates: HashMap<String, String>,
}

impl TemplateManager {
    fn new() -> Self {
        let mut templates = HashMap::new();
        
        // Templates para patrones específicos
        templates.insert("deref_expr".to_string(), "mem::replace({var})".to_string());
        templates.insert("assign_to_deref".to_string(), "safe_assign({var}, {expr})".to_string());
        templates.insert("raw_addr_expr".to_string(), "safe_raw_addr({var})".to_string());
        templates.insert("fn_returns_raw_pointer".to_string(), "/* WARNING: Function returns raw pointer */".to_string());
        templates.insert("raw_pointer_type".to_string(), "{var}".to_string());
        templates.insert("binary_arith_expr".to_string(), "safe_pointer_arithmetic({expr})".to_string());
        templates.insert("array_index_expr".to_string(), "safe_index({expr})".to_string());
        templates.insert("mutable_ref_expr".to_string(), "safe_mut_ref({var})".to_string());
        
        Self { templates }
    }

    fn get_template(&self, pattern_kind: &str) -> Option<&String> {
        self.templates.get(pattern_kind)
    }
}

/// ============================================================================
/// VALIDACIÓN DE MORFOLOGÍA - Análisis de estructura sintáctica
/// ============================================================================
/// Valida que la estructura sintáctica del bloque unsafe coincida con la
/// morfología esperada del patrón, sin depender del contenido específico
fn validate_morphology(expr_unsafe: &ExprUnsafe, pattern_kind: &str) -> bool {
    let block = &expr_unsafe.block;

    match pattern_kind {
        "deref_expr" => {
            // Morfología: bloque con expresión de dereference (*var)
            if block.stmts.len() != 1 {
                return false;
            }
            if let syn::Stmt::Expr(Expr::Unary(ExprUnary { op: UnOp::Deref(_), .. }), _) = &block.stmts[0] {
                return true;
            }
            false
        }
        "assign_to_deref" => {
            // Morfología: bloque con asignación a dereference (*ptr = value)
            if block.stmts.len() != 1 {
                return false;
            }
            if let syn::Stmt::Expr(Expr::Assign(assign), _) = &block.stmts[0] {
                matches!(&*assign.left, Expr::Unary(ExprUnary { op: UnOp::Deref(_), .. }))
            } else {
                false
            }
        }
        "raw_addr_expr" => {
            // Morfología: bloque con expresión de dirección bruta (&raw const/mut var)
            if block.stmts.len() != 1 {
                return false;
            }
            if let syn::Stmt::Expr(Expr::RawAddr(_), _) = &block.stmts[0] {
                return true;
            }
            false
        }
        "raw_pointer_type" => {
            // Morfología: bloque con un identificador de tipo puntero bruto (*const T / *mut T)
            if block.stmts.is_empty() {
                return false;
            }
            // Verifica que contenga al menos una expresión de tipo
            for stmt in &block.stmts {
                if let syn::Stmt::Expr(Expr::Path(_), _) = stmt {
                    return true;
                }
            }
            false
        }
        "binary_arith_expr" => {
            // Morfología: bloque con operación aritmética binaria
            if block.stmts.len() != 1 {
                return false;
            }
            if let syn::Stmt::Expr(Expr::Binary(_), _) = &block.stmts[0] {
                return true;
            }
            false
        }
        "array_index_expr" => {
            // Morfología: bloque con indexación de array [idx]
            if block.stmts.len() != 1 {
                return false;
            }
            if let syn::Stmt::Expr(Expr::Index(_), _) = &block.stmts[0] {
                return true;
            }
            false
        }
        "mutable_ref_expr" => {
            // Morfología: bloque con referencia mutable (&mut var)
            if block.stmts.len() != 1 {
                return false;
            }
            if let syn::Stmt::Expr(Expr::Reference(expr_ref), _) = &block.stmts[0] {
                expr_ref.mutability.is_some()
            } else {
                false
            }
        }
        "unsafe_block" => {
            // Morfología: simplemente un bloque unsafe sin restricción específica
            true
        }
        _ => {
            // Para patrones desconocidos, acepta si tiene al menos una expresión
            !block.stmts.is_empty()
        }
    }
}

/// Extrae elementos dinámicos del bloque unsafe basado en el tipo de patrón
fn extract_dynamic_elements(expr_unsafe: &ExprUnsafe, pattern_kind: &str) -> HashMap<String, String> {
    let mut elements = HashMap::new();
    let block = &expr_unsafe.block; 

    match pattern_kind {
        "deref_expr" => {
            // Busca expresiones de dereference (*var) y extrae 'var'
            for stmt in &block.stmts {
                if let syn::Stmt::Expr(Expr::Unary(ExprUnary { op: UnOp::Deref(_), expr, .. }), _) = stmt {
                    if let Expr::Path(ExprPath { path, .. }) = &**expr {
                        if let Some(ident) = path.get_ident() {
                            elements.insert("var".to_string(), ident.to_string());
                        }
                    }
                }
            }
        }
        "assign_to_deref" => {
            // Para asignaciones como *ptr = value, extrae 'ptr' y 'value'
            for stmt in &block.stmts {
                if let syn::Stmt::Expr(Expr::Assign(assign), _) = stmt {
                    if let Expr::Unary(ExprUnary { op: UnOp::Deref(_), expr, .. }) = &*assign.left {
                        if let Expr::Path(ExprPath { path, .. }) = &**expr {
                            if let Some(ident) = path.get_ident() {
                                elements.insert("var".to_string(), ident.to_string());
                            }
                        }
                    }
                    elements.insert("expr".to_string(), assign.right.to_token_stream().to_string());
                }
            }
        }
        "raw_addr_expr" => {
            // Para &raw const/mut var, extrae 'var'
            for stmt in &block.stmts {
                if let syn::Stmt::Expr(Expr::RawAddr(raw_addr), _) = stmt {
                    elements.insert("var".to_string(), raw_addr.expr.to_token_stream().to_string());
                }
            }
        }
        _ => {
            // Para otros patrones, intenta extraer el contenido del bloque
            if !block.stmts.is_empty() {
                let first_stmt = &block.stmts[0];
                if let syn::Stmt::Expr(expr, _) = first_stmt {
                    elements.insert("expr".to_string(), expr.to_token_stream().to_string());
                }
            }
        }
    }

    elements
}

/// Visitor mutante que aplica transformaciones basadas en patrones detectados
struct PatternBasedModifier<'a> {
    templates: &'a TemplateManager,
    patterns: &'a [PatternInfo],
}

impl<'a> PatternBasedModifier<'a> {
    fn new(templates: &'a TemplateManager, patterns: &'a [PatternInfo]) -> Self {
        Self { templates, patterns }
    }

    /// Encuentra el patrón correspondiente a una expresión unsafe
    /// Valida tanto la posición/contenido como la morfología esperada
    fn find_matching_pattern(&self, expr_unsafe: &ExprUnsafe) -> Option<&PatternInfo> {
        let span = expr_unsafe.span();
        let line = span.start().line;
        let column = span.start().column;
        let unsafe_str = expr_unsafe.to_token_stream().to_string();

        // Busca por línea y columna primero, validando morfología
        if let Some(pattern) = self.patterns
            .iter()
            .find(|p| p.line == line && p.column == column)
        {
            if validate_morphology(expr_unsafe, &pattern.kind) {
                return Some(pattern);
            }
        }

        // Si no encuentra por posición, busca por contenido, validando morfología
        self.patterns
            .iter()
            .find(|p| {
                validate_morphology(expr_unsafe, &p.kind)
                    && (p.snippet.contains(&unsafe_str) || unsafe_str.contains(&p.snippet))
            })
    }
}

impl<'a> VisitMut for PatternBasedModifier<'a> {
    fn visit_expr_mut(&mut self, node: &mut Expr) {
        syn::visit_mut::visit_expr_mut(self, node);

        if let Expr::Unsafe(expr_unsafe) = node {
            if let Some(pattern) = self.find_matching_pattern(expr_unsafe) {
                // Validación adicional: verifica que la morfología sea correcta
                if !validate_morphology(expr_unsafe, &pattern.kind) {
                    return; // No procesa si la morfología no coincide - deja el unsafe intacto
                }

                if let Some(template) = self.templates.get_template(&pattern.kind) {
                    // Extrae elementos dinámicos
                    let elements = extract_dynamic_elements(expr_unsafe, &pattern.kind);

                    // Reemplaza placeholders en el template
                    let mut replacement_code = template.clone();
                    for (key, value) in &elements {
                        replacement_code =
                            replacement_code.replace(&format!("{{{}}}", key), value);
                    }

                    // Si el patrón es "unsafe_block" y no hay template específico,
                    // preserva el contenido
                    if pattern.kind == "unsafe_block" && replacement_code.contains('{') {
                        // Extrae las sentencias del bloque
                        let statements = &expr_unsafe.block.stmts;

                        if statements.len() == 1 {
                            if let syn::Stmt::Expr(expr, _) = &statements[0] {
                                *node = expr.clone();
                            }
                        } else if !statements.is_empty() {
                            let block = expr_unsafe.block.clone();
                            *node = syn::parse_quote!({
                                #block
                            });
                        } else {
                            *node = syn::parse_quote!(());
                        }
                    } else if let Ok(new_expr) = syn::parse_str::<Expr>(&replacement_code) {
                        *node = new_expr;
                    } else {
                        // Fallback: preserva el bloque si no puede parsear el reemplazo
                        let block = expr_unsafe.block.clone();
                        *node = syn::parse_quote!({
                            #block
                        });
                    }
                } else {
                    // Sin template para este patrón, preserva el contenido
                    let statements = &expr_unsafe.block.stmts;

                    if statements.len() == 1 {
                        if let syn::Stmt::Expr(expr, _) = &statements[0] {
                            *node = expr.clone();
                        }
                    } else if !statements.is_empty() {
                        let block = expr_unsafe.block.clone();
                        *node = Expr::Unsafe(expr_unsafe.clone());
                        // *node = syn::parse_quote!({
                        //     #block
                        // });
                    } else {
                        *node = syn::parse_quote!(());
                    }
                }
            }
            // Si NO encuentra un patrón coincidente, deja el bloque unsafe intacto
        }
    }
}

/// Función principal que procesa archivos Rust transformando código unsafe basado en patrones
pub fn replace_unsafe_code(
    input_dir: &Path,
    output_dir: &Path,
    patterns_dir: Option<&Path>,
) -> Result<()> {
    // Verifica que el directorio de entrada exista
    if !input_dir.is_dir() {
        anyhow::bail!("Input directory does not exist: {}", input_dir.display());
    }

    // Crea el directorio de salida
    fs::create_dir_all(output_dir)
        .with_context(|| format!("creating output directory: {}", output_dir.display()))?;

    let templates = TemplateManager::new();
    let mut all_patterns: HashMap<String, Vec<PatternInfo>> = HashMap::new();
    let mut processed_count = 0;
    let mut error_count = 0;

    // Recorre recursivamente el directorio
    for entry in WalkDir::new(input_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
    {
        let input_file_path = entry.path();
        print!("\nProcessing file: {} ... ", input_file_path.display());

        // Calcula la ruta relativa desde el directorio de entrada
        let relative_path = match input_file_path.strip_prefix(input_dir) {
            Ok(path) => path,
            Err(_) => {
                error_count += 1;
                eprintln!("\x1b[91m✗ Error computing relative path for\x1b[0m {}", input_file_path.display());
                continue;
            }
        };

        // Construye la ruta de salida preservando la estructura de directorios
        let output_file_path = output_dir.join(relative_path);

        // Lee y parsea el archivo
        let source_code = match fs::read_to_string(input_file_path) {
            Ok(code) => code,
            Err(e) => {
                error_count += 1;
                eprintln!("\x1b[91m✗ Error reading\x1b[0m {}: {}", input_file_path.display(), e);
                continue;
            }
        };

        let mut patterns: Vec<PatternInfo> = Vec::new();

        if entry.path().extension().and_then(|f| f.to_str()) == Some("rs") {
            
            let mut ast = match parse_file(&source_code) {
                Ok(ast) => ast,
                Err(e) => {
                    error_count += 1;
                    eprintln!("\x1b[91m✗ Error parsing\x1b[0m {}: {}", input_file_path.display(), e);
                    continue;
                }
            };

            // Detecta patrones
            let mut detector = PatternDetector::new(
                input_file_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("file"),
            );
            detector.visit_file(&ast);
            patterns = detector.into_patterns();
            
            // Agrupa patrones por tipo (solo si se requiere reporte)
            if patterns_dir.is_some() {
                for pattern in &patterns {
                    all_patterns
                    .entry(pattern.kind.clone())
                    .or_insert_with(Vec::new)
                    .push(pattern.clone());
                }
            }
    
            // Aplica transformaciones
            let mut modifier = PatternBasedModifier::new(&templates, &patterns);
            modifier.visit_file_mut(&mut ast);
            
            // Escribe el resultado formateado
            let modified_code = prettyplease::unparse(&ast);
            
            if let Some(parent) = output_file_path.parent() {
                if let Err(e) = fs::create_dir_all(parent) {
                    error_count += 1;
                    eprintln!("\x1b[91m✗ Error creating directory\x1b[0m {}: {}", parent.display(), e);
                    continue;
                }
            }
            
            if let Err(e) = fs::write(&output_file_path, modified_code) {
                error_count += 1;
                eprintln!("\x1b[91m✗ Error writing\x1b[0m {}: {}", output_file_path.display(), e);
                continue;
            }
        
        } else {
            // Si no es un archivo .rs, simplemente cópialo
            if let Some(parent) = output_file_path.parent() {
                if let Err(e) = fs::create_dir_all(parent) {
                    error_count += 1;
                    eprintln!("\x1b[91m✗ Error creating directory\x1b[0m {}: {}", parent.display(), e);
                    continue;
                }
            }

            if let Err(e) = fs::copy(&input_file_path, &output_file_path) {
                error_count += 1;
                eprintln!("\x1b[91m✗ Error copying\x1b[0m {}: {}", input_file_path.display(), e);
                continue;
            }
        }
        processed_count += 1;
        println!(
            "\x1b[92m✓ Processed:\x1b[0m {} ({} patterns)",
            input_file_path.display(),
            patterns.len()
        );
    }

    // Opcionalmente, guarda los patrones detectados
    if let Some(patterns_out) = patterns_dir {
        if let Err(e) = fs::create_dir_all(patterns_out) {
            eprintln!(
                "\x1b[33m⚠ Warning:\x1b[0m Could not create patterns directory {}: {}",
                patterns_out.display(),
                e
            );
        } else {
            let summary = serde_json::json!({
                "patterns_by_kind": all_patterns.iter()
                    .map(|(kind, patterns)| {
                        (kind.clone(), patterns.len())
                    })
                    .collect::<HashMap<String, usize>>(),
                "total_patterns": all_patterns.values().map(|v| v.len()).sum::<usize>(),
            });

            let summary_path = patterns_out.join("summary.json");
            if let Err(e) = fs::write(&summary_path, serde_json::to_string_pretty(&summary)?) {
                eprintln!("\x1b[33m⚠ Warning:\x1b[0m Could not write patterns summary: {}", e);
            } else {
                println!("Wrote patterns summary to {}", summary_path.display());
            }
        }
    }

    println!("\n=== Processing Summary ===");
    println!("\x1b[92mFiles processed successfully\x1b[0m: {}", processed_count);
    println!("\x1b[91mFiles with errors\x1b[0m: {}", error_count);

    if !all_patterns.is_empty() {
        println!(
            "Total patterns by kind: {}",
            all_patterns
                .iter()
                .map(|(k, v)| format!("{}: {}", k, v.len()))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    if error_count > 0 {
        anyhow::bail!("Processing completed with {} errors", error_count);
    }

    Ok(())
}
