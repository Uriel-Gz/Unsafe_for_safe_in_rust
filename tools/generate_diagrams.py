#!/usr/bin/env python3
# Generador de diagramas PlantUML para rmorph
# - Genera .puml para: activity, sequence (main.rs) y components (src/)
# - Descarga PNGs desde https://www.plantuml.com/plantuml/png/<encoded>
import argparse, os, re, sys, zlib
from pathlib import Path
from urllib.request import urlopen, Request
from urllib.error import URLError, HTTPError

MAP = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-_"

def encode6bit(b): return MAP[b]

def plantuml_encode(s: str) -> str:
    data = zlib.compress(s.encode("utf-8"))
    data = data[2:-4]
    res = []
    i = 0
    while i < len(data):
        b1 = data[i]
        b2 = data[i+1] if i+1 < len(data) else 0
        b3 = data[i+2] if i+2 < len(data) else 0
        c1 = (b1 >> 2) & 0x3F
        c2 = ((b1 & 0x3) << 4) | ((b2 >> 4) & 0xF)
        c3 = ((b2 & 0xF) << 2) | ((b3 >> 6) & 0x3)
        c4 = b3 & 0x3F
        res += [encode6bit(c1), encode6bit(c2), encode6bit(c3), encode6bit(c4)]
        i += 3
    return "".join(res)

def download_png_from_text(text: str, out_path: str) -> bool:
    enc = plantuml_encode(text)
    url = f"https://www.plantuml.com/plantuml/png/{enc}"
    try:
        req = Request(url, headers={"User-Agent":"plantuml-client"})
        with urlopen(req, timeout=30) as resp:
            data = resp.read()
        with open(out_path, "wb") as fh:
            fh.write(data)
        print(f"Saved PNG: {out_path}")
        return True
    except Exception as e:
        print(f"Error downloading PNG from PlantUML: {e}", file=sys.stderr)
        return False

def write_file(path: str, content: str):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as fh:
        fh.write(content)
    print(f"Wrote: {path}")

def resolve_path(project_root: str, path: str):
    if not path:
        return None
    p = Path(path).expanduser()
    if p.is_absolute():
        return p
    candidate = Path(project_root) / p
    if candidate.exists():
        return candidate
    if p.exists():
        return p
    return candidate

def generate_activity_puml(project_root: str, outdir: str, activity_path: str=None, module_path: str=None):
    source_path = activity_path or module_path
    if source_path:
        source_path = Path(source_path)
        base_name = source_path.stem
        target = os.path.join(outdir, f"activity_{base_name}.puml")
        if source_path.exists():
            templates = {
                "modifier": [
                    "@startuml",
                    "|Archivo|",
                    "start",
                    f":Leer módulo {source_path.name};",
                    ":Parsear código (syn::parse_file);",
                    ":Detectar patrones (PatternDetector::visit_file);",
                    ":Extraer elementos dinámicos (extract_dynamic_elements);",
                    ":Buscar template (TemplateManager::get_template);",
                    "if (template encontrado?) then (sí)",
                    "  :Reemplazar placeholders en template;",
                    "  :Parsear reemplazo (syn::parse_str);",
                    "  :Aplicar reemplazo al AST (VisitMut);",
                    "else (no)",
                    "  :Marcar como `unsafe_block` genérico;",
                    "endif",
                    ":Formatear AST (prettyplease::unparse);",
                    ":Escribir archivo de salida (fs::write);",
                    "stop",
                    "@enduml"
                ],
                "extractor": [
                    "@startuml",
                    "|Archivo|",
                    "start",
                    f":Leer módulo {source_path.name};",
                    ":Parsear código (syn::parse_file);",
                    ":Detectar bloques unsafe;",
                    ":Extraer elementos unsafe;",
                    ":Guardar código extraído y metadatos;",
                    ":Formatear resultados;",
                    ":Escribir datos de extracción;",
                    "stop",
                    "@enduml"
                ],
                "suggestions": [
                    "@startuml",
                    "|Archivo|",
                    "start",
                    f":Leer módulo {source_path.name};",
                    ":Parsear código (syn::parse_file);",
                    ":Detectar patrones unsafe;",
                    ":Generar sugerencias de mejora;",
                    ":Agrupar sugerencias por tipo;",
                    ":Formatear salida;",
                    "stop",
                    "@enduml"
                ]
            }
            content = "\n".join(templates.get(base_name, [
                "@startuml",
                "|Archivo|",
                "start",
                f":Leer módulo {source_path.name};",
                ":Parsear código (syn::parse_file);",
                ":Detectar flujo y estructuras del módulo;",
                ":Resumir comportamiento relevante;",
                ":Formatear resultado final;",
                "stop",
                "@enduml"
            ]))
        else:
            content = """@startuml
|Archivo|
start
:Leer archivo (fs::read_to_string);
:Parsear código (syn::parse_file);
:Detectar patrones (PatternDetector::visit_file);
:Extraer elementos dinámicos (extract_dynamic_elements);
:Buscar template (TemplateManager::get_template);
if (template encontrado?) then (sí)
  :Reemplazar placeholders en template;
  :Parsear reemplazo (syn::parse_str);
  :Aplicar reemplazo al AST (VisitMut);
else (no)
  :Marcar como `unsafe_block` genérico;
endif
:Formatear AST (prettyplease::unparse);
:Escribir archivo de salida (fs::write);
stop
@enduml
"""
    else:
        tpl = os.path.join(project_root, "Documentacion", "diagrams", "activity_pipeline.puml")
        if os.path.exists(tpl):
            content = open(tpl, encoding="utf-8").read()
        else:
            content = """@startuml
|Archivo|
start
:Leer archivo (fs::read_to_string);
:Parsear código (syn::parse_file);
:Detectar patrones (PatternDetector::visit_file);
:Extraer elementos dinámicos (extract_dynamic_elements);
:Buscar template (TemplateManager::get_template);
if (template encontrado?) then (sí)
  :Reemplazar placeholders en template;
  :Parsear reemplazo (syn::parse_str);
  :Aplicar reemplazo al AST (VisitMut);
else (no)
  :Marcar como `unsafe_block` genérico;
endif
:Formatear AST (prettyplease::unparse);
:Escribir archivo de salida (fs::write);
stop
@enduml
"""
        target = os.path.join(outdir, "activity_pipeline.puml")
    write_file(target, content)
    return target, content

def generate_sequence_puml(project_root: str, outdir: str, main_path: str=None, module_path: str=None):
    if module_path:
        source_path = module_path
    else:
        source_path = main_path or os.path.join(project_root, "rmorph", "src", "main.rs")
    modules = []
    if os.path.exists(source_path):
        src = open(source_path, encoding="utf-8").read()
        found = re.findall(r'([A-Za-z_][A-Za-z0-9_]*)::[A-Za-z_][A-Za-z0-9_]*\s*\(', src)
        for m in found:
            if m.lower() not in ("std",):
                print(f"Found module: {m}")
                if m not in modules:
                    modules.append(m)
    lines = ["@startuml", "actor User", "participant Main", "participant Executor"]
    for m in modules:
        lines.append(f"participant {m.capitalize()}")
    lines += [
        "",
        "User -> Main: start()",
        "Main -> Main: show_init()",
        "loop Menu",
        "  Main -> Main: display_menu()",
        "  User -> Main: choice",
        "  Main -> Executor: execute_option(choice)"
    ]
    if "extractor" in [m.lower() for m in modules]:
        lines += [
            "  alt choice == \"1\"",
            "    Executor -> Extractor: extract_unsafe_blocks(path, out_dir)",
            "    Extractor -> FileSystem: walk files (WalkDir)",
            "    Extractor -> Extractor: process_file(path)",
            "    Extractor -> PatternDetector: PatternDetector::visit_file(ast)",
            "    PatternDetector --> Extractor: patterns",
            "    Extractor -> FileSystem: write unsafe blocks and metadata",
            "  end"
        ]
    if "modifier" in [m.lower() for m in modules]:
        lines += [
            "  alt choice == \"2\"",
            "    Executor -> Modifier: replace_unsafe_code(path, out_dir_ch)",
            "    Modifier -> FileSystem: walk files (WalkDir)",
            "    Modifier -> PatternDetector: detector.visit_file(ast)",
            "    PatternDetector --> Modifier: patterns",
            "    Modifier -> FileSystem: write transformed files",
            "  end"
        ]
    if "suggestions" in [m.lower() for m in modules]:
        lines += [
            "  alt choice == \"3\"",
            "    Executor -> Suggestions: generate_suggestions(path)",
            "    Suggestions -> PatternDetector: analyze files",
            "    PatternDetector --> Suggestions: patterns",
            "    Suggestions --> User: display_suggestions()",
            "  end"
        ]
    for m in modules:
        if m.lower() not in ("extractor","modifier","suggestions"):
            M = m.capitalize()
            lines += [f"  Executor -> {M}: call {m}::*", f"  {M} --> Executor: Result"]
    lines += ["  Executor --> Main: Result", "end", 'Main -> User: exit when choice=="4"', "@enduml"]
    content = "\n".join(lines)
    target = os.path.join(outdir, "sequence_main.puml")
    write_file(target, content)
    return target, content

def generate_components_puml(project_root: str, outdir: str, src_dir: str=None, module_path: str=None):
    def make_alias(name: str, reserved: set[str]) -> str:
        alias = re.sub(r'[^A-Za-z0-9_]', '_', name)
        if not alias:
            alias = 'Node'
        alias = alias[0].upper() + alias[1:]
        alias_lower = alias.lower()
        original = alias
        i = 1
        while alias_lower in reserved:
            i += 1
            alias = f"{original}_{i}"
            alias_lower = alias.lower()
        reserved.add(alias_lower)
        return alias

    if module_path:
        source_path = Path(module_path)
        base_name = source_path.stem
        target = os.path.join(outdir, f"components_{base_name}.puml")
        lines = ['@startuml']
        reserved_aliases = set()
        internal_modules = {os.path.splitext(f)[0] for f in os.listdir(os.path.join(project_root, 'rmorph', 'src')) if f.endswith('.rs')}
        external_whitelist = {
            'syn', 'walkdir', 'serde', 'serde_json', 'anyhow',
            'quote', 'proc_macro2', 'regex', 'serde_derive'
        }
        ignore_refs = {
            'vec', 'hashmap', 'path', 'pathbuf', 'result', 'option', 'string',
            'fs', 'self', 'super', 'crate', 'write', 'read', 'file', 'iter',
            'vecdeque', 'box', 'rc', 'arc', 'str', 'bool', 'u8', 'u32', 'u64',
            'usize', 'println', 'format', 'from'
        }
        if source_path.exists():
            try:
                txt = source_path.read_text(encoding="utf-8")
            except:
                txt = ""
            functions = re.findall(r'fn\s+([A-Za-z_][A-Za-z0-9_]*)', txt)
            structs = re.findall(r'struct\s+([A-Za-z_][A-Za-z0-9_]*)', txt)
            items = []
            if functions:
                items += [f"{fn}()" for fn in functions[:8]]
                if len(functions) > 8:
                    items.append(f"+{len(functions)-8} funciones más")
            if structs:
                items += [f"struct {st}" for st in structs[:6]]
                if len(structs) > 6:
                    items.append(f"+{len(structs)-6} structs más")
            if not items:
                items = ["No hay símbolos públicos detectados"]
            node_alias = make_alias(base_name, reserved_aliases)
            label = "\n".join(items)
            lines.append(f'package "{base_name}" {{')
            lines.append(f'  [{base_name}.rs|{label}] as {node_alias}')
            lines.append('}')
        else:
            txt = ""
            node_alias = make_alias(base_name, reserved_aliases)
            lines.append(f'package "{base_name}" {{')
            lines.append(f'  [{base_name}.rs] as {node_alias}')
            lines.append('}')

        refs = set()
        for line in txt.splitlines():
            line = line.strip()
            m = re.match(r'^use\s+(.+?);', line)
            if m:
                path = m.group(1).strip()
                path = re.sub(r'\s+as\s+[A-Za-z_][A-Za-z0-9_]*$', '', path)
                path = re.sub(r'^pub\s+', '', path)
                if path.startswith('crate::'):
                    path = path[len('crate::'):]
                first = path.split('::', 1)[0]
                refs.add(first)

        for r in re.findall(r'\b([A-Za-z_][A-Za-z0-9_]*)::', txt):
            refs.add(r)

        refs_unique = []
        seen_lower = set()
        for r in sorted(refs, key=str.lower):
            rl = r.lower()
            if rl in (base_name.lower(), 'std', 'self', 'super', 'crate'):
                continue
            if rl in seen_lower or rl in ignore_refs:
                continue
            if rl in internal_modules or rl in external_whitelist:
                seen_lower.add(rl)
                refs_unique.append(r)

        deps = []
        dep_nodes = []
        for ref in refs_unique:
            dep_alias = make_alias(ref, reserved_aliases)
            dep_label = f"{ref}.rs" if ref.lower() in internal_modules else ref
            dep_nodes.append(f'  [{dep_label}] as {dep_alias}')
            deps.append((node_alias, dep_alias, ref))

        if dep_nodes:
            lines[1:1] = dep_nodes
        else:
            lines.append(f'note right of {node_alias}: Sin dependencias externas')

        for _, dep_alias, ref in sorted(deps, key=lambda x: x[2].lower()):
            lines.append(f'{node_alias} --> {dep_alias} : uses')
    else:
        if not src_dir:
            src_dir = os.path.join(project_root, "rmorph", "src")
        modules = []
        if os.path.exists(src_dir):
            for e in os.listdir(src_dir):
                if e.endswith(".rs"):
                    modules.append(e)
        lines = ['@startuml', 'package "rmorph::src" {']
        for m in modules:
            lines.append(f'  [{m}] as {os.path.splitext(m)[0].capitalize()}')
        lines.append("}")
        deps = set()
        for m in modules:
            mname = os.path.splitext(m)[0]
            path = os.path.join(src_dir, m)
            try:
                txt = open(path, encoding="utf-8").read()
            except:
                txt = ""
            for other in modules:
                othername = os.path.splitext(other)[0]
                if othername != mname and re.search(rf'\b{othername}::', txt):
                    deps.add((mname.capitalize(), othername.capitalize()))
    if not module_path:
        for a,b in sorted(deps):
            lines.append(f"{a} --> {b} : uses")
    lines.append("@enduml")
    content = "\n".join(lines)
    if not module_path:
        target = os.path.join(outdir, "components_src.puml")
    write_file(target, content)
    return target, content

def ensure_outdir(path: str):
    os.makedirs(path, exist_ok=True)
    return path

def main():
    ap = argparse.ArgumentParser(description="Generador de diagramas PlantUML para rmorph")
    ap.add_argument("-t","--type", choices=["activity","sequence","components","all"], default="all")
    ap.add_argument("-p","--project", default=None, help="Ruta raíz del proyecto (por defecto: padre de tools/)")
    ap.add_argument("--main", help="Ruta a main.rs para generar el diagrama de secuencia (opcional)")
    ap.add_argument("--module-path", help="Ruta a un archivo de módulo Rust para generar secuencia específica (opcional)")
    ap.add_argument("--activity-path", help="Ruta a un archivo de módulo Rust para generar diagrama de actividad específico (opcional)")
    ap.add_argument("--components-path", help="Ruta a un archivo de módulo Rust para generar diagrama de componentes específico (opcional)")
    ap.add_argument("-o","--outdir", default=None, help="Directorio de salida (por defecto Documentacion/diagrams)")
    ap.add_argument("--no-png", action="store_true", help="No descargar PNGs desde plantuml.com")
    args = ap.parse_args()

    script_dir = os.path.dirname(os.path.abspath(__file__))
    project_root = args.project or os.path.abspath(os.path.join(script_dir, ".."))
    outdir = args.outdir or os.path.join(project_root, "Documentacion", "diagrams")
    ensure_outdir(outdir)

    activity_path = resolve_path(project_root, args.activity_path) if args.activity_path else None
    module_path = resolve_path(project_root, args.module_path) if args.module_path else None
    main_path = resolve_path(project_root, args.main) if args.main else None
    components_path = resolve_path(project_root, args.components_path) if args.components_path else None

    generated = []
    if args.type in ("activity","all"):
        p, txt = generate_activity_puml(project_root, outdir, activity_path=activity_path, module_path=module_path)
        generated.append((p, txt))
    if args.type in ("sequence","all"):
        p, txt = generate_sequence_puml(project_root, outdir, main_path=main_path, module_path=module_path)
        generated.append((p, txt))
    if args.type in ("components","all"):
        p, txt = generate_components_puml(project_root, outdir, module_path=components_path or module_path)
        generated.append((p, txt))

    if not args.no_png:
        for p, txt in generated:
            png_path = os.path.splitext(p)[0] + ".png"
            ok = download_png_from_text(txt, png_path)
            if not ok:
                print(f"Failed to download PNG for {p}. You can run tools/plantuml_encode.py or check network.")

    print("Done. Generated puml files:")
    for p,_ in generated:
        print(" -", p)

if __name__ == "__main__":
    main()
