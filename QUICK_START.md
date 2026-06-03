# Quick Start — Uso rápido (Rust)

Guía mínima para ejecutar el flujo principal con la implementación en Rust (`rmorph`).

Requisitos mínimos
- Rust toolchain (`rustc`, `cargo`) instalado y en `PATH`.

Pasos rápidos
1. Clonar el repositorio y situarse en la raíz:

```bash
git clone <URL-del-repo>
cd Unsafe_for_safe_in_rust
```

2. Compilar y ejecutar (modo interactivo):

```bash
# Compilar
cargo build --manifest-path rmorph/Cargo.toml

# Ejecutar (menu interactivo)
cargo run --manifest-path rmorph/Cargo.toml
```

3. En el menú seleccione:
- `1` para extraer bloques `unsafe` (genera `result/` con `unsafe_blocks`, `unsafe_ast`, `patterns`, `html_patterns`, ...)
- `2` para intentar reemplazos automáticos (genera `result_changed/`)
- `3` para generar sugerencias de código seguro

4. Ejecutar sobre una carpeta de ejemplo (opcional):

```bash
# use la opción 1 y luego ingrese: rmorph/examples
```

Notas rápidas
- Los scripts Python en `code_src/` están obsoletos: prefiera `rmorph/`.
- Para producción, compile en release: `cargo build --release --manifest-path rmorph/Cargo.toml`.

Contacto y versión
- Autor: Uriel Gz — https://github.com/Uriel-Gz
- Versión del quick start: 2026-06-03
