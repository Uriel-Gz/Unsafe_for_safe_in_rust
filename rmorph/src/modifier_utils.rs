use std::path::Path;
use syn::{Expr, ExprPath, ExprRawAddr, ExprUnary, ExprUnsafe, UnOp};

pub fn calc_relative_path<'a>(
    input_file_path: &'a Path,
    input_dir: &'a Path,
    error_count: &mut usize,
) -> Option<&'a Path> {
    if input_dir.is_file() {
        return input_file_path.file_name().map(Path::new);
    }

    let relative_path = match input_file_path.strip_prefix(
        input_dir
            .is_file()
            .then(|| input_dir.parent().unwrap_or(input_dir))
            .unwrap_or(input_dir),
    ) {
        Ok(path) => path,
        Err(_) => {
            *error_count += 1;
            eprintln!(
                "\x1b[91m✗ Error computing relative path for\x1b[0m {}",
                input_file_path.display()
            );
            return None;
        }
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
            if let Some(syn::Stmt::Expr(
                Expr::Unary(ExprUnary {
                    op: UnOp::Deref(_), ..
                }),
                _,
            )) = block.stmts.last()
            {
                return true;
            }
            false
        }
        "assign_to_deref" => {
            for stmt in &block.stmts {
                if let syn::Stmt::Expr(Expr::Assign(assign), _) = stmt {
                    if let Expr::Unary(ExprUnary {
                        op: UnOp::Deref(_), ..
                    }) = &*assign.left
                    {
                        return true;
                    }
                }
            }
            false
        }
        "raw_addr_expr" => block
            .stmts
            .iter()
            .any(|stmt| matches!(stmt, syn::Stmt::Expr(Expr::RawAddr(ExprRawAddr { .. }), _))),
        // "mutable_ref_expr" => {
        //     if block.stmts.len() != 1 {
        //         return false;
        //     }
        //     if let syn::Stmt::Expr(Expr::Reference(expr_ref), _) = &block.stmts[0] {
        //         expr_ref.mutability.is_some()
        //     } else {
        //         false
        //     }
        // }
        "unsafe_block" => !block.stmts.is_empty(),
        "matching_call_omission" => {
            // Verifica si dentro del bloque unsafe hay una llamada a `Some(...)`
            for stmt in &block.stmts {
                if let syn::Stmt::Expr(Expr::Call(call), _) = stmt {
                    if let Expr::Path(ExprPath { path, .. }) = &*call.func {
                        for seg in &path.segments {
                            if seg.ident == "Some" {
                                return true;
                            }
                        }
                    }
                }
            }
            false
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{calc_relative_path, validate_morphology};
    use syn::parse_str;

    #[test]
    fn calculates_relative_path_for_directory_input() {
        let input_dir = std::path::Path::new("/workspace/project");
        let input_file = std::path::Path::new("/workspace/project/src/main.rs");
        let mut errors = 0;

        assert_eq!(
            calc_relative_path(input_file, input_dir, &mut errors),
            Some(std::path::Path::new("src/main.rs"))
        );
        assert_eq!(errors, 0);
    }

    #[test]
    fn calculates_file_name_when_input_is_a_file() {
        let input_dir = std::path::PathBuf::from(format!(
            "target/rmorph-modifier-utils-{}-main.rs",
            std::process::id()
        ));
        std::fs::write(&input_dir, "fn main() {}").unwrap();
        let mut errors = 0;

        assert_eq!(
            calc_relative_path(&input_dir, &input_dir, &mut errors),
            input_dir.file_name().map(std::path::Path::new)
        );
        assert_eq!(errors, 0);
        let _ = std::fs::remove_file(input_dir);
    }

    #[test]
    fn rejects_empty_deref_block_without_panicking() {
        let expression: syn::ExprUnsafe = parse_str("unsafe {} ").unwrap();
        assert!(!validate_morphology(&expression, "deref_expr"));
    }

    #[test]
    fn recognizes_raw_address_expression() {
        let expression: syn::ExprUnsafe = parse_str("unsafe { &raw const value }").unwrap();
        assert!(validate_morphology(&expression, "raw_addr_expr"));
    }
}
