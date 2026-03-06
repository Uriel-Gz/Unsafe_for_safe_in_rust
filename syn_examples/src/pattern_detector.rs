use anyhow::Result;
use proc_macro2::Span;
use quote::ToTokens;
use serde::Serialize;
use std::fs;
use std::path::Path;
use syn::visit::Visit;
use syn::spanned::Spanned;
use serde_json; 
use syn::{Expr, ExprAssign, ExprUnsafe, ItemFn, Type, ExprUnary, UnOp};

#[derive(Serialize)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct PatternInfo {
    pub file: String,
    pub kind: String,
    pub index: usize,
    pub line: usize,
    pub column: usize,
    pub snippet: String,
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

    pub fn clone(&self) -> Self {
        Self {
            patterns: self.patterns.clone(),
            file_stem: self.file_stem.clone(),
            counter: self.counter,
        }
    }

    pub fn new(file_stem: &str) -> Self {
        Self { patterns: Vec::new(), file_stem: file_stem.to_string(), counter: 0 }
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
        });
    }

    pub fn save_to(&self, out_dir: &Path) -> Result<()> {
        let dir = out_dir.join("patterns");
        fs::create_dir_all(&dir)?;
        let fname = format!("{}_patterns.json", self.file_stem);
        let path = dir.join(fname);
        let json = serde_json::to_string_pretty(&self.patterns)?;
        fs::write(&path, json)?;
        println!("Wrote patterns to {}", path.display());
        Ok(())
    }

    /// Consume detector and return collected patterns, removing duplicates/nested ones
    pub fn into_patterns(mut self) -> Vec<PatternInfo> {
        self.patterns = self.filter_nested_patterns();
        self.patterns
    }

    /// Filtra patrones anidados o duplicados en la misma línea
    /// Mantiene los patrones más amplios y elimina los que están contenidos dentro
    fn filter_nested_patterns(&self) -> Vec<PatternInfo> {
        let mut filtered = Vec::new();

        for pattern in &self.patterns {
            let mut should_add = true; 

            // Verifica si este patrón está contenido dentro de otro existente en la misma línea
            for other in &self.patterns {
                if pattern.line != other.line && pattern == other {
                    continue;
                }

                // Si el snippet de 'other' contiene el de 'pattern' y están en la misma línea,
                // el patrón más pequeño es redundante
                if other.snippet.contains(&pattern.snippet) && other.snippet.len() > pattern.snippet.len() || 
                   pattern.snippet.contains(&other.snippet) && pattern.snippet.len() > other.snippet.len(){
                    should_add = false;
                    break;
                }
            }

            if should_add {
                // También verifica que no esté duplicado en el resultado filtrado
                let is_duplicate = filtered.iter().any(|p: &PatternInfo| {
                    p.line == pattern.line 
                        || p.column == pattern.column 
                        && p.kind == pattern.kind
                        && p.snippet == pattern.snippet
                });

                if !is_duplicate {
                    filtered.push(pattern.clone());
                }
            }
        }

        filtered
    }

    pub fn save_html(&self, out_path: &Path, path: &Path) -> Result<()> {
        let dir = out_path.join("html_patterns");
        fs::create_dir_all(&dir)?;
        let fname = format!("{}_patterns.html", self.file_stem);
        let path_to = dir.join(fname);
        //  bloque html con la información requerida

        let mut safe_content = String::new();
        for (i, pat) in self.patterns.clone().iter().enumerate() {
            let filename = format!("{}", self.file_stem);
            let file_path = path.display(); // f]or JS compatibility
            let unsafe_id = format!("unsafe_block_{}_{}", self.file_stem, pat.index);
            let path_to_file = path.parent()
                                                    .and_then(|p| p.to_str())
                                                    .unwrap_or("")
                                                    .replace("\\", "/"); // for JS compatibility

            let pre_content: String = format!(
                "<h3>In the repository (subfolder/s) {}</h3>\nIn the file: <a onclick=\"cargarArchivo('../../{}','{}','{}')\"><em>{}</em></a> linea {} columna {}\n<code class=\"rust\" style=\"border-radius: 10px;\">\n",
                path_to_file, file_path, unsafe_id, pat.line, filename , pat.line, pat.column
            );
            let pos_content: String = format!(
                "</code>\n<code class=\"rust\" id=\"{}\" style=\"overflow: auto; height: 150px; display: none;\"></code>\n",
                unsafe_id
            );
            safe_content.push_str(&pre_content);
            safe_content.push_str(&pat.snippet);
            safe_content.push_str(&pos_content);
        }

        let mut res = String::new();
        res.push_str(HEADER);
        res.push_str(&safe_content);
        res.push_str(FOOTER);

        fs::write(&path_to, res)?;
        println!("Wrote patterns to {}", path_to.display());
        Ok(())
    }
}

impl<'ast> Visit<'ast> for PatternDetector {
    fn visit_expr_unsafe(&mut self, node: &'ast ExprUnsafe) {
        // Guarda el contador actual antes de visitar el contenido del bloque
        let patterns_before = self.patterns.len();
        
        // Continúa visitando el contenido del bloque unsafe
        syn::visit::visit_expr_unsafe(self, node);
        
        // Si no se detectaron patrones específicos dentro del bloque,
        // registra el "unsafe_block" genérico
        if self.patterns.len() == patterns_before {
            let tok = node.to_token_stream().to_string();
            self.push("unsafe_block", node.unsafe_token.span(), tok);
        }
    }

    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        if let syn::ReturnType::Type(_, ty) = &node.sig.output {
            if matches!(&**ty, Type::Ptr(_)) {
                // use the fn ident span for reporting
                let span = node.sig.ident.span();
                let tok = node.sig.ident.to_string();
                self.push("fn_returns_raw_pointer", span, tok);
            }
        }
        syn::visit::visit_item_fn(self, node);
    }

    fn visit_expr(&mut self, node: &'ast Expr) {
        if let Expr::Unary(ExprUnary { op: UnOp::Deref(_), expr: inner, .. }) = node {
            let tok = inner.to_token_stream().to_string();
            self.push("deref_expr", node.span(), tok);
        }
        syn::visit::visit_expr(self, node);
    }

    fn visit_expr_assign(&mut self, node: &'ast ExprAssign) {
        // left side can be a unary deref: *p = x
        if let Expr::Unary(ExprUnary { op: UnOp::Deref(_), expr: _, .. }) = &*node.left {
            self.push("assign_to_deref", node.left.span(), node.left.to_token_stream().to_string());
        }
        syn::visit::visit_expr_assign(self, node);
    }

    // Verifica expresiones unarias como negación o dereferencia adicional
    fn visit_expr_unary(&mut self, node: &'ast ExprUnary) {
        // Ejemplo: detectar negación de punteros o valores críticos
        if matches!(node.op, UnOp::Not(_)) {
            let tok = node.to_token_stream().to_string();
            self.push("unary_not_expr", node.span(), tok);
        }
        syn::visit::visit_expr_unary(self, node);
    }

    // Verifica tipos puntero raw en declaraciones
    fn visit_type_ptr(&mut self, node: &'ast syn::TypePtr) {
        let tok = node.to_token_stream().to_string();
        self.push("raw_pointer_type", node.span(), tok);
        syn::visit::visit_type_ptr(self, node);
    }

    // Verifica expresiones de dirección raw (&raw const o &raw mut)
    fn visit_expr_raw_addr(&mut self, node: &'ast syn::ExprRawAddr) {
        let tok = node.to_token_stream().to_string();
        self.push("raw_addr_expr", node.span(), tok);
        syn::visit::visit_expr_raw_addr(self, node);
    }

    // Verifica llamadas a funciones, potencialmente unsafe
    fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {
        // Ejemplo: detectar llamadas con argumentos potencialmente peligrosos
        if node.args.len() > 5 {
            let tok = node.to_token_stream().to_string();
            self.push("large_call_args", node.span(), tok);
        }
        syn::visit::visit_expr_call(self, node);
    }

    // Verifica indexaciones de arrays o slices
    fn visit_expr_index(&mut self, node: &'ast syn::ExprIndex) {
        // Ejemplo: detectar indexación sin bounds checking explícito
        let tok = node.to_token_stream().to_string();
        self.push("array_index_expr", node.span(), tok);
        syn::visit::visit_expr_index(self, node);
    }

    // Verifica expresiones de referencia (& o &mut)
    fn visit_expr_reference(&mut self, node: &'ast syn::ExprReference) {
        // Ejemplo: detectar referencias mutables a datos sensibles
        if node.mutability.is_some() {
            let tok = node.to_token_stream().to_string();
            self.push("mutable_ref_expr", node.span(), tok);
        }
        syn::visit::visit_expr_reference(self, node);
    }

    // Verifica operaciones binarias, como aritmética de punteros
    fn visit_expr_binary(&mut self, node: &'ast syn::ExprBinary) {
        // Ejemplo: detectar operaciones aritméticas potencialmente inseguras
        if matches!(node.op, syn::BinOp::Add(_) | syn::BinOp::Sub(_)) {
            let tok = node.to_token_stream().to_string();
            self.push("binary_arith_expr", node.span(), tok);
        }
        syn::visit::visit_expr_binary(self, node);
    }

}
