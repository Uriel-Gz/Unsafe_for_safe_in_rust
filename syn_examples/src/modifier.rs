use crate::pattern_detector;
use anyhow::{Context, Result};
use pattern_detector::PatternDetector;
use proc_macro2::{Span, TokenStream, TokenTree};
use quote::ToTokens;
use serde::Serialize;
use serde_json::{self, to_string};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use syn::spanned::Spanned;
use syn::{visit::Visit, visit_mut::VisitMut, Expr, ExprUnsafe, File};
use walkdir::WalkDir;

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
    fn visit_expr(&mut self, node: &'ast Expr) {
        // Save the token stream (includes the `unsafe` keyword and braces) and span
        self.blocks.push((node.to_token_stream(), node.span()));
        // Continue traversal
        syn::visit::visit_expr(self, node);
    }
}


pub fn modify(directory: &Path, out_dir: &Path) -> Result<()> {
    // collector for all patterns across files; we'll aggregate by kind at the end
    let mut all_patterns: Vec<pattern_detector::PatternInfo> = Vec::new();

    for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rs") {
            let src =
                fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
            let mut ast = match syn::parse_file(&src) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("warning: skipping {} (parse error: {})", path.display(), e);
                    return Ok(());
                }
            };

            // Collect all unsafe blocks for batch analysis and write them separately
            let mut collector = UnsafeCollector::new();
            collector.visit_file(&ast);
            if !collector.blocks.is_empty() {
                let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("file");
                let unsafe_dir = out_dir.join("unsafe_blocks").join(path).join(stem);
                let unsafe_ast_dir = out_dir.join("unsafe_ast").join(path).join(stem);
                fs::create_dir_all(&unsafe_dir)?;
                fs::create_dir_all(&unsafe_ast_dir)?;

                for (i, (blk, span)) in collector.blocks.into_iter().enumerate() {
                    // Convert tokenstream to string. This preserves the `unsafe { ... }` text.
                    let content = blk.to_string();
                    let fname = format!("{}_unsafe_{}.rs", stem, i + 1);
                    let fpath = unsafe_dir.join(&fname);
                    fs::write(&fpath, &content)
                        .with_context(|| format!("writing {}", fpath.display()))?;
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
                    fs::write(&meta_path, meta_json)
                        .with_context(|| format!("writing {}", meta_path.display()))?;
                    // println!("Wrote unsafe AST metadata to {}", meta_path.display());
                }
            }

            // Detect patterns using pattern_detector and return patterns
            let mut detector =
                PatternDetector::new(path.file_stem().and_then(|s| s.to_str()).unwrap_or("file"));
            detector.visit_file(&ast);
            let _ = detector.save_to(out_dir)?; // optional: save per-file patterns
            let _ = detector.save_html(out_dir, path)?; // save HTML representation
            let mut patterns = detector.into_patterns();

            all_patterns.append(&mut patterns);
        }
    }

    // aggregate by kind
    let mut by_kind: HashMap<String, Vec<pattern_detector::PatternInfo>> = HashMap::new();
    for p in all_patterns {
        by_kind.entry(p.kind.clone()).or_default().push(p);
    }

    Ok(())
}
