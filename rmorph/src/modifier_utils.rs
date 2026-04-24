use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use syn::{Expr, ExprUnary, ExprUnsafe, File, UnOp};
use crate::pattern_detector::PatternInfo;



pub fn calc_relative_path<'a>(input_file_path: &'a Path, input_dir: &'a Path, error_count: &mut usize) -> Option<&'a Path> {
    let relative_path = match input_file_path.strip_prefix(input_dir.is_file()
        .then(|| input_dir.parent()
        .unwrap_or(input_dir))
        .unwrap_or(input_dir)) {
        Ok(path) => path,
        Err(_) => {
            *error_count += 1;
            eprintln!("\x1b[91m✗ Error computing relative path for\x1b[0m {}", input_file_path.display());
            return None;}
    };
    Some(relative_path)
}

pub fn show_sumary(processed_count: usize, error_count: usize, patterns_by_kind: &HashMap<String, Vec<PatternInfo>>) {
    println!("\n\x1b[93m_____ Processing Summary _____\x1b[0m\n");
    println!("\x1b[92mFiles processed successfully\x1b[0m: {}", processed_count);
    println!("\x1b[91mFiles with errors\x1b[0m: {}", error_count);

    if !patterns_by_kind.is_empty() {
        println!(
            "\x1b[30mTotal patterns by kind\x1b[0m: {}",
            patterns_by_kind
                .iter()
                .map(|(k, v)| format!("\x1b[33m{}:\x1b[0m {}", k, v.len()))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
}

pub fn create_sumary_file(patterns_by_kind: &HashMap<String, Vec<PatternInfo>>, patterns_out: &Path) -> Result<(), Box<dyn std::error::Error>>{
    let summary = serde_json::json!({
            "patterns_by_kind": patterns_by_kind.iter()
                .map(|(kind, patterns)| {
                    (kind.clone(), patterns.len())
                })
                .collect::<HashMap<String, usize>>(),
            "total_patterns": patterns_by_kind.values().map(|v| v.len()).sum::<usize>(),
        });
    let summary_path = patterns_out.join("summary.json");
    if let Err(e) = fs::write(&summary_path, serde_json::to_string_pretty(&summary)?) {
        eprintln!("\x1b[33m⚠ Warning:\x1b[0m Could not write patterns summary: {}", e);
    } else {
        println!("Wrote patterns summary to {}", summary_path.display());
    }
    Ok(())
}

/// ============================================================================
/// VALIDACIÓN DE MORFOLOGÍA - Análisis y verificación de estructura sintáctica
/// ============================================================================
pub fn validate_morphology(expr_unsafe: &ExprUnsafe, pattern_kind: &str) -> bool {
    let block = &expr_unsafe.block;

    match pattern_kind {
        "deref_expr" => {           
            if let syn::Stmt::Expr(Expr::Unary(ExprUnary { op: UnOp::Deref(_), .. }), _) = &block.stmts[block.stmts.len()-1] {
                return true;
            }
            false
        }
        "assign_to_deref" => {
            for stmt in &block.stmts {
                 if let syn::Stmt::Expr(Expr::Assign(assign), _) = stmt {
                    if let Expr::Unary(ExprUnary { op: UnOp::Deref(_), .. }) = &*assign.left {
                        return true;
                    }
                }
            }
            return false;
        }
        "raw_addr_expr" => {
            if let Some(last) = block.stmts.last() {
                match last {
                    syn::Stmt::Expr(expr,_) => {
                        if let Expr::Unary(ExprUnary { op: UnOp::Deref(_), .. }) = expr {
                            return true;
                        }
                    }
                    _ => {}
                }
            }
            false
        }
        "mutable_ref_expr" => {
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
            !block.stmts.is_empty()
        }
        _ => false
    }
}

