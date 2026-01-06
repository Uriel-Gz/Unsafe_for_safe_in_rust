use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use syn::{visit_mut::VisitMut, visit::Visit, File, Expr, ExprUnsafe};
use syn::spanned::Spanned;
use proc_macro2::{TokenTree, TokenStream, Span};
use quote::ToTokens;
use serde::Serialize;
use serde_json::{self, to_string};
mod pattern_detector;
use pattern_detector::PatternDetector;
use std::collections::HashMap;
use walkdir::WalkDir;
mod extractor;
mod modifier;
use modifier::modify;
use extractor::{process_file, extract_unsafe_blocks};

struct UnsafeReplacer;

impl VisitMut for UnsafeReplacer {
    fn visit_expr_mut(&mut self, node: &mut Expr) {
        // Recurse first
        syn::visit_mut::visit_expr_mut(self, node);

        // If this expression is an unsafe block, we want to rewrite it
        if let Expr::Unsafe(u) = node {
            // Convert the unsafe block to tokens, replace the 'unsafe' ident
            let mut tokens = u.to_token_stream().into_iter().collect::<Vec<TokenTree>>();
            // Find first ident token equal to "unsafe" and replace it
            for i in 0..tokens.len() {
                match &tokens[i] {
                    TokenTree::Ident(id) => {
                        if id.to_string() == "unsafe" {
                            tokens[i] = TokenTree::Ident(proc_macro2::Ident::new("unp_reemplazado", id.span()));
                            break;
                        }
                    }
                    _ => {}
                }
            }

            // Re-parse tokens into an Expr
            let ts: TokenStream = TokenStream::from_iter(tokens.into_iter());
            if let Ok(new_expr) = syn::parse2::<Expr>(ts) {
                *node = new_expr;
            }
        }
    }
}


fn main() -> Result<()> {
    let directory = PathBuf::from("examples");
    let out_dir = PathBuf::from("result");
    let out_dir_ch = PathBuf::from("result_changed");
    fs::create_dir_all(&out_dir)?;
    fs::create_dir_all(&out_dir_ch)?;

    //extractor de codigo en formato de bloques unsafe y html
    extract_unsafe_blocks(&directory, &out_dir)?;

    //modificador de codigo, reemplaza unsafe por unp_reemplazado
    modify(&directory, &out_dir_ch)?;

    Ok(())
}
