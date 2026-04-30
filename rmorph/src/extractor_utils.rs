use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use syn::{Expr, ExprUnary, ExprUnsafe, File, UnOp};
use crate::pattern_detector::PatternInfo;

pub fn create_sumary_file(patterns_by_kind: &HashMap<String, Vec<PatternInfo>>, patterns_out: &Path) -> Result<(), Box<dyn std::error::Error>>{
    let out_path = patterns_out.join("report");
    let summary_path = out_path.join("summary.json");
    
    if let Err(e) = fs::create_dir_all(out_path) {
        eprintln!("\x1b[33m⚠ Warning:\x1b[0m Could not create patterns directory: {}", e);
    } else {
        let summary = serde_json::json!({
                "patterns_by_kind": patterns_by_kind.iter()
                    .map(|(kind, patterns)| {
                        (kind.clone(), patterns.len())
                    })
                    .collect::<HashMap<String, usize>>(),
                "total_patterns": patterns_by_kind.values().map(|v| v.len()).sum::<usize>(),
            });
            
        let summary_json = serde_json::to_string_pretty(&summary)?;
        if let Err(e) = fs::write(&summary_path, summary_json) {
            eprintln!("\x1b[33m⚠ Warning:\x1b[0m Could not write patterns summary: {}", e);
        } else {
            println!("Wrote patterns summary to {}", summary_path.display());
        }
    }
    Ok(())
}

pub fn create_grouped_by_kind_file(patterns_by_kind: &HashMap<String, Vec<PatternInfo>>, patterns_out: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // aggregate by kind
    let mut by_kind: HashMap<String, Vec<PatternInfo>> = HashMap::new();
    for p in patterns_by_kind.values().flatten() {
        by_kind.entry(p.kind.clone()).or_default().push(p.clone());
    }

    let kind_dir = patterns_out.join("patterns");
    fs::create_dir_all(&kind_dir)?;
    
    let agg_path = kind_dir.join("#_grouped_by_kind.json");
    let agg_json = serde_json::to_string_pretty(&by_kind)?;

    if let Err(e) = fs::write(&agg_path, agg_json) {
        eprintln!("\x1b[33m⚠ Warning:\x1b[0m Could not write grouped patterns file: {}", e);
    } else {
        println!("Wrote grouped patterns to {}", agg_path.display());
    }
    Ok(())
}