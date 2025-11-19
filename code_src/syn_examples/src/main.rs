use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use syn::{visit_mut::VisitMut, visit::Visit, File, Expr, ExprUnsafe};
use syn::spanned::Spanned;
use proc_macro2::{TokenTree, TokenStream, Span};
use quote::ToTokens;
use serde::Serialize;
use serde_json;

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

/// Visitor that collects all `unsafe { ... }` expression token streams.
struct UnsafeCollector {
    // store tokenstream plus the span of the `unsafe` token for location info
    blocks: Vec<(TokenStream, Span)>,
}

impl UnsafeCollector {
    fn new() -> Self {
        Self { blocks: Vec::new() }
    }
}

impl<'ast> Visit<'ast> for UnsafeCollector {
    fn visit_expr_unsafe(&mut self, node: &'ast ExprUnsafe) {
        // Save the token stream (includes the `unsafe` keyword and braces) and span
        self.blocks.push((node.to_token_stream(), node.unsafe_token.span()));
        // Continue traversal
        syn::visit::visit_expr_unsafe(self, node);
    }
}

fn process_file(path: &Path, out_dir: &Path) -> Result<()> {
    let src = fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let mut ast: File = syn::parse_file(&src).expect("parsing file failed");

    // Dump AST debug information to out/ast_debug/<file>_ast.txt
    dump_ast_debug(path, &ast, out_dir)?;

    // 1) Run the replacement visitor (keeps previous behavior)
    let mut visitor = UnsafeReplacer;
    visitor.visit_file_mut(&mut ast);

    // 2) Pretty-print and write the modified file to out/
    let new = prettyplease::unparse(&ast);
    let rel = path.file_name().unwrap();
    let out_path = out_dir.join(rel);
    fs::write(&out_path, new).with_context(|| format!("writing {}", out_path.display()))?;
    println!("Wrote {}", out_path.display());

    // 3) Collect all unsafe blocks for batch analysis and write them separately
    let mut collector = UnsafeCollector::new();
    collector.visit_file(&ast);
    if !collector.blocks.is_empty() {
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("file");
        let unsafe_dir = out_dir.join("unsafe_blocks").join(stem);
        let unsafe_ast_dir = out_dir.join("unsafe_ast").join(stem);
        fs::create_dir_all(&unsafe_dir)?;
        fs::create_dir_all(&unsafe_ast_dir)?;

        for (i, (blk, span)) in collector.blocks.into_iter().enumerate() {
            // Convert tokenstream to string. This preserves the `unsafe { ... }` text.
            let content = blk.to_string();
            let fname = format!("{}_unsafe_{}.rs", stem, i + 1);
            let fpath = unsafe_dir.join(&fname);
            fs::write(&fpath, &content).with_context(|| format!("writing {}", fpath.display()))?;
            println!("Extracted unsafe block to {}", fpath.display());

            // Write a small metadata JSON file with span/location and the token stream
            #[derive(Serialize)]
            struct UnsafeMeta<'a> {
                file: &'a str,
                index: usize,
                line: usize,
                column: usize,
                tokens: &'a str,
            }

            let meta = {
                let loc = span.start();
                UnsafeMeta {
                    file: stem,
                    index: i + 1,
                    line: loc.line,
                    column: loc.column,
                    tokens: &content,
                }
            };

            let meta_json = serde_json::to_string_pretty(&meta)?;
            let meta_fname = format!("{}_unsafe_{}.meta.json", stem, i + 1);
            let meta_path = unsafe_ast_dir.join(&meta_fname);
            fs::write(&meta_path, meta_json).with_context(|| format!("writing {}", meta_path.display()))?;
            println!("Wrote unsafe AST metadata to {}", meta_path.display());
        }
    }
    Ok(())
}

/// Write a debug representation of the parsed AST to `out/ast_debug/<stem>_ast.txt`.
fn dump_ast_debug(path: &Path, ast: &File, out_dir: &Path) -> Result<()> {
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("file");
    let ast_dir = out_dir.join("ast_debug");
    fs::create_dir_all(&ast_dir)?;
    let fname = format!("{}_ast.txt", stem);
    let fpath = ast_dir.join(fname);
    // `syn::File` no implementa `Debug` en algunas versi   ones de `syn`.
    // Usamos `prettyplease::unparse` para obtener una representación en código fuente.
    let pretty = prettyplease::unparse(ast);
    fs::write(&fpath, pretty).with_context(|| format!("writing {}", fpath.display()))?;
    println!("Wrote AST (pretty source) to {}", fpath.display());
    Ok(())
}

fn main() -> Result<()> {
    let examples = PathBuf::from("examples");
    let out_dir = PathBuf::from("out");
    fs::create_dir_all(&out_dir)?;

    for entry in fs::read_dir(&examples)? {
        let p = entry?.path();
        if p.extension().map(|s| s == "rs").unwrap_or(false) {
            process_file(&p, &out_dir)?;
        }
    }

    Ok(())
}
