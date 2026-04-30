use anyhow::{Context, Result};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use syn::visit::Visit;
use syn::spanned::Spanned;
use syn::visit_mut::VisitMut;
use syn::{Expr, ExprBlock, ExprPath, ExprUnary, ExprUnsafe, File, UnOp, parse_file};
use walkdir::WalkDir;
use crate::pattern_detector::{PatternDetector, PatternInfo};
use crate::modifier_utils::{calc_relative_path, validate_morphology};
use crate::config::INTO_UNSAFE_BLOCKS;

/// Gestor de templates para reemplazos seguros
struct TemplateManager {
    templates: HashMap<String, String>,
}

impl TemplateManager {
    fn new() -> Self {
        let mut templates = HashMap::new();
        
        templates.insert("deref_expr".to_string(), "Box::new(var)".to_string());
        templates.insert("assign_to_deref".to_string(), "mem::replace(var, expr)".to_string());
        templates.insert("raw_addr_expr".to_string(), "safe_raw_addr(var)".to_string());
        templates.insert("matching_call_omission".to_string(), "match var { Ok(val) => Some(val),\n None => None }".to_string());
        
        Self { templates }
    }

    fn get_template(&self, pattern_kind: &str) -> Option<&String> {
        self.templates.get(pattern_kind)
    }
}

/// Extrae elementos dinámicos del bloque unsafe basado en el tipo de patrón
fn extract_dynamic_elements(expr_unsafe: &ExprUnsafe, pattern_kind: &str) -> HashMap<String, String> {
    let mut elements = HashMap::new();
    let block = &expr_unsafe.block; 

    match pattern_kind {
        "deref_expr" => {
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
            for stmt in &block.stmts {
                if let syn::Stmt::Expr(Expr::RawAddr(raw_addr), _) = stmt {
                    elements.insert("var".to_string(), raw_addr.expr.to_token_stream().to_string());
                }
            }
        }
        "mutable_ref_expr" => {
            for stmt in &block.stmts {
                if let syn::Stmt::Expr(Expr::Reference(expr_ref), _) = stmt {
                    if expr_ref.mutability.is_some() {
                        if let Expr::Unary(ExprUnary { op: UnOp::Deref(_), expr, .. }) = &*expr_ref.expr {
                            if let Expr::Paren(expr_paren) = &**expr {
                                if let Expr::Cast(cast_expr) = &*expr_paren.expr {
                                    if let Expr::Path(ExprPath { path, .. }) = &*cast_expr.expr {
                                        if let Some(ident) = path.get_ident() {
                                            elements.insert("var".to_string(), ident.to_string());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        "matching_call_omission" => {
            for stmt in &block.stmts {
                if let syn::Stmt::Expr(Expr::Call(call_expr), _) = stmt {
                    if let Expr::Path(ExprPath { path, .. }) = &*call_expr.func {
                        if path.is_ident("Some") {
                            if let Some(arg) = call_expr.args.first() {
                                if let Expr::Unary(ExprUnary { op: UnOp::Deref(_), expr, .. }) = arg {
                                    if let Expr::Path(ExprPath { path, .. }) = &**expr {
                                        if let Some(ident) = path.get_ident() {
                                            elements.insert("var".to_string(), ident.to_string());
                                        }
                                    }
                                }                            
                            }
                        }
                    }
                }
            }
        }
        _ => {
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
    
    fn modify_unsafe_block(expr: &mut Expr, replacement_code: &str, pattern: &PatternInfo) -> () {
        let mut real_snip = pattern.snippet.clone();
        let new_block = pattern.localblock.replace( &real_snip, &replacement_code);
        if let Ok(block) = syn::parse_str::<syn::ExprBlock>(&new_block) {
            *expr = block.into();
        } else {}
    }
}

impl<'a> VisitMut for PatternBasedModifier<'a> {
    fn visit_expr_mut(&mut self, node: &mut Expr) {
        syn::visit_mut::visit_expr_mut(self, node);

        if let Expr::Unsafe(expr_unsafe) = node {
            if let Some(pattern) = self.find_matching_pattern(expr_unsafe) {

                if let Some(template) = self.templates.get_template(&pattern.kind) {
                    let elements = extract_dynamic_elements(expr_unsafe, &pattern.kind);

                    // Reemplaza placeholders en el template
                    let mut replacement_code = template.clone();
                    for (key, value) in &elements {
                        replacement_code =
                            replacement_code.replace(key, value);   
                    }

                    if let Ok(new_expr) = syn::parse_str::<syn::Stmt>(&replacement_code){
                        PatternBasedModifier::<'a>::modify_unsafe_block(node, &replacement_code, &pattern);
                    } else if let Ok(new_expr) = syn::parse_str::<syn::Expr>(&replacement_code){
                        PatternBasedModifier::<'a>::modify_unsafe_block(node, &replacement_code, &pattern);
                    }else{
                        // preserva el bloque en caso de no poder parsear el código de reemplazo
                        let block = expr_unsafe.block.clone();
                        *node = Expr::Unsafe(expr_unsafe.clone());
                    }
                } else {
                    let statements = &expr_unsafe.block.stmts;

                    if !statements.is_empty() && statements.len() == 1 {
                        if let syn::Stmt::Expr(expr, _) = &statements[0] {
                            *node = expr.clone();
                        }
                    } else {
                        let block = expr_unsafe.block.clone();
                        *node = syn::parse_quote!({
                            #block
                        });
                    }
                }
            }
        }
    }
}

// Función principal que procesa archivos Rust transformando código unsafe basado en patrones
pub fn replace_unsafe_code(input_dir: &Path, output_dir: &Path) -> Result<()> {

    if !input_dir.exists() {
        anyhow::bail!("Input directory does not exist: {}", input_dir.display());
    }

    fs::create_dir_all(output_dir)
        .with_context(|| format!("creating output directory: {}", output_dir.display()))?;

    let templates = TemplateManager::new();
    let mut processed_count = 0;
    let mut error_count = 0;

    for entry in WalkDir::new(input_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
    {
        let input_file_path = entry.path();
        let relative_pth = calc_relative_path(input_file_path, input_dir, &mut error_count).unwrap();
        let output_file_path = output_dir.join(relative_pth);

        let source_code = match fs::read_to_string(input_file_path) {
            Ok(code) => code,
            Err(e) => {
                error_count += 1;
                eprintln!("\x1b[91m✗ Error reading\x1b[0m {}: {}", input_file_path.display(), e);
                continue;
            }
        };

        let mut patterns: Vec<PatternInfo> = Vec::new();
        let mut final_code = source_code.clone();
        let mut ast;

        if entry.path().extension().and_then(|f| f.to_str()) == Some("rs") {
            ast = match parse_file(&source_code) {
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
                
            // Aplica las modificaciones basadas en patrones detectados
            let mut modifier = PatternBasedModifier::new(&templates, &patterns);
            modifier.visit_file_mut(&mut ast);
            
            // formatea el código modificado
            final_code = prettyplease::unparse(&ast);
        }
            
        if let Some(parent) = output_file_path.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                error_count += 1;
                eprintln!("\x1b[91m✗ Error creating directory\x1b[0m {}: {}", parent.display(), e);
                continue;
            }
        }
        
        if let Err(e) = fs::write(&output_file_path, final_code) {
            error_count += 1;
            eprintln!("\x1b[91m✗ Error writing\x1b[0m {}: {}", output_file_path.display(), e);
            continue;
        }

        processed_count += 1;
        println!(
            "\x1b[92m✓ Processed:\x1b[0m {} ({} patterns)",
            input_file_path.display(),
            patterns.len()
        );
    }

    if error_count > 0 {
        anyhow::bail!("Processing completed with {} errors", error_count);
    }

    Ok(())
}
