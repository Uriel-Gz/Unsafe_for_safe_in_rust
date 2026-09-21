use crate::config::{CANT_BLOCKS, INTO_UNSAFE_BLOCKS};
use anyhow::Result;
use proc_macro2::Span;
use quote::ToTokens;
use serde::Serialize;
use serde_json;
use std::fs;
use std::path::Path;
use syn::spanned::Spanned;
use syn::visit::Visit;
use syn::{Expr, ExprAssign, ExprUnary, ExprUnsafe, UnOp};

#[derive(Serialize, Clone, PartialEq)]
pub struct PatternInfo {
    pub file: String,
    pub kind: String,
    pub index: usize,
    pub line: usize,
    pub column: usize,
    pub snippet: String,
    pub localblock: String,
}

#[derive(Clone)]
pub struct PatternDetector {
    pub patterns: Vec<PatternInfo>,
    file_stem: String,
    counter: usize,
}

//  directivas de estilo para lenguajes
const HEADER: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Document</title>
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.5.1/styles/default.min.css" />
    <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.5.1/highlight.min.js"></script>
    <script>hljs.highlightAll();</script>
    <style>
        a:hover{
            cursor: pointer;
        }
    </style>
</head>
<body>
<pre>
"#;

// comportamiendo: busqueda e insercion de código que se observa
const FOOTER: &str = r#"
</pre>
    <script>
        let f = false;
        function cargarArchivo(archivo, id, line) {
            if (!f) {
                f = true;
                fetch(archivo)
                    .then(response => {
                        if (!response.ok) {
                            throw new Error('Error al cargar el archivo');
                        }
                        return response.text();
                    })
                    .then(data => {
                        const bloque = document.getElementById(id);
                        bloque.innerText = data;
                        bloque.style.display = 'block'; // Mostrar el contenido
                        bloque.scrollTop = parseInt(line, 10) * 15; // Mostrar el contenido
                    })
                    .catch(error => {
                        console.error('Error:', error);
                    });
                }else{
                    f = false;
                    const bloque = document.getElementById(id);
                    bloque.innerText = '';
                    bloque.style.display = 'none'; // Mostrar el contenido
                }
            }
    </script>
</body>
</html>
"#;

impl PatternDetector {
    pub fn new(file_stem: &str) -> Self {
        Self {
            patterns: Vec::new(),
            file_stem: file_stem.to_string(),
            counter: 0,
        }
    }

    fn push(&mut self, kind: &str, span: Span, tok: String) {
        self.counter += 1;
        let loc = span.start();
        self.patterns.push(PatternInfo {
            file: self.file_stem.clone(),
            kind: kind.to_string(),
            index: self.counter,
            line: loc.line,
            column: loc.column,
            snippet: tok,
            localblock: String::new(),
        });
    }

    fn push_block(&mut self, block: String) {
        for i in (0..self.patterns.len()).rev() {
            if self.patterns[i].localblock.is_empty() {
                self.patterns[i].localblock = block.clone();
            } else {
                // Asumimos que los bloques están anidados, así que si encontramos uno con bloque ya asignado,
                // los anteriores también lo tendrán
                break;
            }
        }
    }

    pub fn save_to(&self, out_dir: &Path) -> Result<()> {
        let dir = out_dir.join("patterns");
        fs::create_dir_all(&dir)?;
        let fname = format!("{}_patterns.json", self.file_stem);
        let path = dir.join(fname);
        let json = serde_json::to_string_pretty(&self.patterns)?;
        fs::write(&path, json)?;
        // println!("_\x1b[34m Writing JSON\x1b[0m patterns info to: {} ", path.display());
        Ok(())
    }

    pub fn save_html(&self, out_path: &Path, path: &Path) -> Result<()> {
        let dir = out_path.join("html_patterns").join(path.parent().unwrap());
        fs::create_dir_all(&dir)?;

        let fname = format!("{}_patterns.html", self.file_stem);
        let path_to = dir.join(fname);

        let mut safe_content = String::new();
        for pat in self.patterns.clone().iter() {
            let filename = self.file_stem.to_string();
            let file_path = path.display();
            let unsafe_id = format!("unsafe_block_{}_{}", self.file_stem, pat.index);
            let path_to_file = path
                .parent()
                .and_then(|p| p.to_str())
                .unwrap_or("")
                .replace("\\", "/");

            let pre_content: String = format!(
                "<h3>In the repository (subfolder/s) {}</h3>\nIn the file: <a onclick=\"cargarArchivo('../../{}','{}','{}')\">\
                <em>{}</em></a> linea {} columna {}\n<code class=\"rust\" style=\"border-radius: 10px;\">\n",
                path_to_file, file_path, unsafe_id, pat.line, filename , pat.line, pat.column
            );
            let pos_content: String = format!(
                "</code>\n<code class=\"rust\" id=\"{}\" style=\"overflow: auto; height: 150px; display: none;\"></code>\n",
                unsafe_id
            );
            safe_content.push_str(&pre_content);
            safe_content.push_str(&pat.localblock);
            safe_content.push_str(&pos_content);
        }

        let mut res = String::new();
        res.push_str(HEADER);
        res.push_str(&safe_content);
        res.push_str(FOOTER);

        fs::write(&path_to, res)?;
        // println!("_\x1b[34m Writing HTML\x1b[0m patterns info to: {} ", path_to.display());
        Ok(())
    }

    pub fn into_patterns(self) -> Vec<PatternInfo> {
        self.patterns
    }

    // Filtra patrones anidados o duplicados en la misma línea, dejando el patrón más largo (más externo) por línea
    pub fn filter_nested_patterns(&mut self) -> Result<()> {
        use std::collections::HashMap;
        let mut line_to_pattern: HashMap<usize, PatternInfo> = HashMap::new();

        for pat in &self.patterns {
            let clean_snippet = pat
                .snippet
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>();
            if let Some(existing) = line_to_pattern.get(&pat.line) {
                let existing_clean = existing
                    .snippet
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect::<String>();
                if clean_snippet.len() > existing_clean.len() {
                    line_to_pattern.insert(pat.line, pat.clone());
                }
                // Si el nuevo es más corto o igual, se ignora (se queda el existente)
            } else {
                line_to_pattern.insert(pat.line, pat.clone());
            }
        }

        self.patterns = line_to_pattern.into_values().collect();
        Ok(())
    }
}

impl<'ast> Visit<'ast> for PatternDetector {
    fn visit_expr_unsafe(&mut self, node: &'ast ExprUnsafe) {
        unsafe {
            INTO_UNSAFE_BLOCKS = true;
            CANT_BLOCKS += 1;
            let patterns_before = self.patterns.len();

            syn::visit::visit_expr_unsafe(self, node);

            // Si no se detectaron patrones específicos dentro del bloque,
            // registra el "unsafe_block" genérico
            if self.patterns.len() == patterns_before {
                let tok = node.to_token_stream().to_string();
                self.push("unsafe_block", node.unsafe_token.span(), tok);
            }

            self.push_block(node.block.to_token_stream().to_string());

            INTO_UNSAFE_BLOCKS = false;
        }
    }

    fn visit_expr(&mut self, node: &'ast Expr) {
        // Detecta expresiones de dereferencia, como *p o *(expr)
        if let Expr::Unary(ExprUnary {
            op: UnOp::Deref(_),
            expr: inner,
            ..
        }) = &node
        {
            if let Expr::Path(_) = inner.as_ref() {
                let tok = node.to_token_stream().to_string();
                self.push("deref_expr", node.span(), tok);
            } else if let Expr::Paren(_) = inner.as_ref() {
                let tok = node.to_token_stream().to_string();
                self.push("deref_expr", node.span(), tok);
            }
        }
        syn::visit::visit_expr(self, node);
    }

    // Detecta asignaciones a punteros dereferenciados, como *p = x
    fn visit_expr_assign(&mut self, node: &'ast ExprAssign) {
        // left side can be a unary deref: *p = x
        if let Expr::Unary(ExprUnary {
            op: UnOp::Deref(_),
            expr: _,
            ..
        }) = &*node.left
        {
            self.push(
                "assign_to_deref",
                node.span(),
                node.to_token_stream().to_string(),
            );
        }
        syn::visit::visit_expr_assign(self, node);
    }

    //TODO Verifica expresiones de dirección raw (&raw const o &raw mut)
    fn visit_expr_raw_addr(&mut self, node: &'ast syn::ExprRawAddr) {
        let tok = node.to_token_stream().to_string();
        self.push("raw_addr_expr", node.span(), tok);
        syn::visit::visit_expr_raw_addr(self, node);
    }

    //* Verifica llamadas a funciones, potencialmente unsafe (usada en el ejemplo de Some())
    fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
        unsafe {
            if INTO_UNSAFE_BLOCKS {
                if let Expr::Path(syn::ExprPath {
                    attrs: _,
                    qself: _,
                    path,
                }) = &*i.func
                {
                    for ph in &path.segments {
                        if ph.ident == "Some" {
                            let tok = i.to_token_stream().to_string();
                            self.push("matching_call_omission", i.span(), tok);
                        }
                    }
                }
            }
        }
        syn::visit::visit_expr_call(self, i);
    }

    //< Verifica expresiones de referencia (& o &mut)
    fn visit_expr_reference(&mut self, node: &'ast syn::ExprReference) {
        // Ejemplo: detectar referencias mutables a datos sensibles
        unsafe {
            if INTO_UNSAFE_BLOCKS && node.mutability.is_some() {
                let tok = node.to_token_stream().to_string();
                self.push("mutable_ref_expr", node.span(), tok);
            }
        }
        syn::visit::visit_expr_reference(self, node);
    }
}

#[cfg(test)]
mod tests {
    use super::PatternDetector;
    use syn::{parse_file, visit::Visit};

    #[test]
    fn detects_deref_and_assignment_patterns_inside_unsafe() {
        let file = parse_file(
            "fn main() { let p = std::ptr::null_mut::<i32>(); unsafe { *p = 1; let _x = *p; } }",
        )
        .unwrap();
        let mut detector = PatternDetector::new("sample");

        detector.visit_file(&file);

        let patterns = detector.into_patterns();
        assert!(patterns
            .iter()
            .any(|pattern| pattern.kind == "assign_to_deref"));
        assert!(patterns.iter().any(|pattern| pattern.kind == "deref_expr"));
    }

    #[test]
    fn detects_raw_address_and_mutable_reference_patterns() {
        let file = parse_file(
            "fn main() { let mut value = 1; unsafe { let _ptr = &raw mut value; let _ref = &mut value; } }",
        )
        .unwrap();
        let mut detector = PatternDetector::new("sample");

        detector.visit_file(&file);

        let patterns = detector.into_patterns();
        assert!(patterns
            .iter()
            .any(|pattern| pattern.kind == "raw_addr_expr"));
        assert!(patterns
            .iter()
            .any(|pattern| pattern.kind == "mutable_ref_expr"));
    }
}
