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

