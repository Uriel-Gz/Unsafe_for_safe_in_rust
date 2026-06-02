use anyhow::Result;
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::fs;
use syn::{parse_file, File, visit::Visit};
use walkdir::WalkDir;

use crate::pattern_detector::{PatternDetector, PatternInfo};

/// Estructura para almacenar sugerencias de mejora
#[derive(Debug, Clone)]
#[derive(Serialize)]
pub struct Suggestion {
    pub pattern_kind: String,
    pub code_snippet: String,
    pub suggestion_text: String,
    pub file: String,
    pub line: usize,
    pub column: usize,
}

/// Generador de sugerencias basado en patrones detectados
pub struct SuggestionGenerator {
    suggestions: Vec<Suggestion>,
}

impl SuggestionGenerator {
    pub fn new() -> Self {
        Self { suggestions: Vec::new() }
    }

    /// Analiza un archivo y genera sugerencias
    pub fn analyze_file(&mut self, file_path: &Path) -> Result<()> {
        let source_code = fs::read_to_string(file_path)?;
        let ast = parse_file(&source_code)?;
        
        let file_stem = file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("file");

        let mut detector = PatternDetector::new(file_stem);
        detector.visit_file(&ast);
        let patterns = detector.into_patterns();

        for pattern in patterns {
            self.add_suggestion_for_pattern(&pattern);
        }

        Ok(())
    }

    /// Analiza todos los archivos en un directorio
    pub fn analyze_directory(&mut self, dir_path: &Path) -> Result<()> {
        for entry in WalkDir::new(dir_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().and_then(|f| f.to_str()) == Some("rs"))
        {
            if let Err(e) = self.analyze_file(entry.path()) {
                eprintln!("Warning: Could not analyze {}: {}", entry.path().display(), e);
            }
        }
        Ok(())
    }

    /// Agrega una sugerencia específica según el tipo de patrón
    fn add_suggestion_for_pattern(&mut self, pattern: &PatternInfo) {
        let suggestion_text = match pattern.kind.as_str() {
            "deref_expr" => {
                "Considere usar iteradores o métodos seguros de std::ptr \n\x1b[1;32m  <> Es posible modificar con la herramienta\x1b[0m".to_string()
            }
            "assign_to_deref" => {
                "Evite asignaciones directas a memoria dereferenciada \n\x1b[1;32m  <> Es posible modificar con la herramienta\x1b[0m".to_string()
            }
            "raw_addr_expr" => {
                "Las expresiones &raw son de bajo nivel. \nConsidere referencias seguras (&T) o (&mut T) cuando sea posible. \n\x1b[91m  >> No es posible modificar con la herramienta\x1b[0m".to_string()
            }
            "matching_call_omission" => {
                "Considere usar match explícito en lugar de Some(x?) \n\x1b[1;32m  <> Es posible modificar con la herramienta\x1b[0m".to_string()
            }
            "mutable_ref_expr" => {
                "Verifique si Cell<T> o RefCell<T> podrían ser utiles a su necesidad \n\x1b[91m  >> No es posible modificar con la herramienta\x1b[0m".to_string()
            }
            "unsafe_block" => {
                "No se encontraron similitudes con patrones conocidos, sin sugerencia, consulte la documentacion en caso de trabajar con memoria.".to_string()
            }
            _ => "Revise este patrón de código unsafe.".to_string()
        };

        self.suggestions.push(Suggestion {
            pattern_kind: pattern.kind.clone(),
            code_snippet: pattern.snippet.clone(),
            suggestion_text,
            file: pattern.file.clone(),
            line: pattern.line,
            column: pattern.column,
        });
    }

    /// Muestra las sugerencias de forma clara y organizada
    pub fn display_suggestions(&self) {
        if self.suggestions.is_empty() {
            println!("\n\x1b[92m✓ No se detectaron patrones unsafe en el código analyzado.\x1b[0m");
            return;
        }

        println!("\n{}", "═".repeat(70));
        println!("\x1b[1;36m║ SUGERENCIAS PARA CÓDIGO UNSAFE\x1b[0m");
        println!("{}\n", "═".repeat(70));

        // Agrupar por tipo de patrón
        let mut by_kind: std::collections::HashMap<String, Vec<&Suggestion>> = 
            std::collections::HashMap::new();
        
        for suggestion in &self.suggestions {
            by_kind
                .entry(suggestion.pattern_kind.clone())
                .or_insert_with(Vec::new)
                .push(suggestion);
        }

        // Mostrar sugerencias por grupo
        for (kind, suggestions) in by_kind.iter() {
            println!("\n\x1b[1;33m┌─ {} ({})\x1b[0m", kind.to_uppercase(), suggestions.len());
            println!("{}", "─".repeat(50));

            for (idx, s) in suggestions.iter().enumerate() {
                println!("\n\x1b[1;32m  [{}/{}] Archivo: {}, Línea {}, Columna {}\x1b[0m", 
                    idx + 1, suggestions.len(), s.file, s.line, s.column);
                println!("  \x1b[90mCódigo:\x1b[0m {}", truncate_code(&s.code_snippet, 60));
            }
            println!("  \x1b[96m→ Sugerencia:\x1b[0m {}", suggestions[0].suggestion_text);
            println!();
        }

        println!("{}", "─".repeat(50));
    }

    /// Exporta las sugerencias a un archivo JSON
    pub fn export_json(&self, output_path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.suggestions)?;
        fs::write(output_path, json)?;
        println!("\x1b[92m✓ Sugerencias exportadas a: {}\x1b[0m", output_path.display());
        Ok(())
    }
}

/// Acorta el código para mostrar en una línea
fn truncate_code(code: &str, max_len: usize) -> String {
    let cleaned = code.replace('\n', " ").replace("  ", " ");
    if cleaned.len() > max_len {
        format!("{}...", &cleaned[..max_len])
    } else {
        cleaned
    }
}

/// Función principal para generar sugerencias desde una ruta
pub fn generate_suggestions(path: &Path) -> Result<SuggestionGenerator> {
    let mut generator = SuggestionGenerator::new();

    if path.is_file() {
        generator.analyze_file(path)?;
    } else if path.is_dir() {
        generator.analyze_directory(path)?;
    } else {
        anyhow::bail!("La ruta especificada no existe: {}", path.display());
    }

    Ok(generator)
}