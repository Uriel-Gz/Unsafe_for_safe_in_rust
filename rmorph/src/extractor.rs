use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use syn::{visit_mut::VisitMut, visit::Visit, File, ExprUnsafe, Expr};
use syn::spanned::Spanned;
use proc_macro2::{TokenTree, TokenStream, Span};
use quote::ToTokens;
use serde::Serialize;
use serde_json::{self, to_string};
use std::collections::HashMap;
use walkdir::WalkDir;
use pattern_detector::PatternDetector;
use crate::extractor_utils::{create_sumary_file, create_grouped_by_kind_file};
use crate::pattern_detector;
use crate::config::CANT_BLOCKS;


#[derive(Serialize)]
struct UnsafeMeta<'a> {
    file: &'a str,
    index: usize,
    line: usize,
    column: usize,
    tokens: &'a str,
}

struct UnsafeCollector {
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
        syn::visit::visit_expr_unsafe(self, node);
    }
}


pub fn process_file(path: &Path, out_dir: &Path) -> Result<Vec<pattern_detector::PatternInfo>> {
    let src = fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let mut ast = match syn::parse_file(&src) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("\x1b[91m warning!:\x1b[0m skipping {} \x1b[91m(parse error: {})\x1b[0m", path.display(), e);
            return Ok(Vec::new());
        }
    };

    let mut collector = UnsafeCollector::new();
    collector.visit_file(&ast);

    if !collector.blocks.is_empty() {
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("file");
        let unsafe_dir = out_dir.join("unsafe_blocks").join(path.parent().unwrap());
        let unsafe_ast_dir = out_dir.join("unsafe_ast").join(path.parent().unwrap());

        fs::create_dir_all(&unsafe_dir)?;
        fs::create_dir_all(&unsafe_ast_dir)?;

        for (i, (blk, span)) in collector.blocks.into_iter().enumerate() {
            let content = blk.to_string();
            let fname = format!("{}_unsafe_{}.rs", stem, i + 1);
            let fpath = unsafe_dir.join(&fname);
            fs::write(&fpath, &content).with_context(|| format!("writing {}", fpath.display()))?;

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
        }
    }

    // Detect patterns using pattern_detector and return patterns
    let mut detector = PatternDetector::new(path.file_stem()
                                                                            .and_then(|s| s.to_str())
                                                                            .unwrap_or("file"));
    detector.visit_file(&ast);
    let _ = detector.filter_nested_patterns()?;

    let _ = detector.save_to(out_dir)?; // optional: save per-file patterns in JSON
    let _ = detector.save_html(out_dir, path)?; // save HTML representation

    let patterns = detector.into_patterns();
    Ok(patterns)
}

pub fn extract_unsafe_blocks(directory: &Path, out_dir: &Path) -> Result<()> {
    let mut all_patterns: Vec<pattern_detector::PatternInfo> = Vec::new();
    let mut all_kinds: HashMap<String, Vec<pattern_detector::PatternInfo>> = HashMap::new();

    for entry in WalkDir::new(directory)
        .into_iter()
        .filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rs") {
            let mut pats = process_file(&path, &out_dir)?;

            for pattern in &pats {
                all_kinds
                .entry(pattern.kind.clone())
                .or_insert_with(Vec::new)
                .push(pattern.clone());
            }

            all_patterns.append(&mut pats);
        }
    }

    print!("\nDesea crear un resumen de los tipos de patrones detectados? (s/n): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if input.trim().to_string() == "s" {
        create_sumary_file(&all_kinds, out_dir);
    }
    unsafe {
        println!("\nTotal de bloques `unsafe` encontrados: {}", CANT_BLOCKS);
        CANT_BLOCKS = 0; // reset counter for next run
    }

    create_grouped_by_kind_file(&all_kinds, out_dir);

    Ok(())
}