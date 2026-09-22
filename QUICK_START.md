# Quick Start — Uso rápido (Rust)

Guía mínima para la versión actual de `rmorph`.

## Requisitos
- Rust toolchain (`rustc`, `cargo`) instalado y disponible en `PATH`.
- Un proyecto Rust o un archivo Rust para analizar.

## 1) Clonar y entrar al repositorio

```bash
git clone <URL-del-repo>
cd <carpeta-del-repo>
```

## 2) Compilar

```bash
cargo build --manifest-path rmorph/Cargo.toml
```

## 3) Ejecutar la herramienta

```bash
cargo run --manifest-path rmorph/Cargo.toml
```

Al iniciarse, el programa muestra un menú interactivo:

- `1` — Extraer código `unsafe`
- `2` — Reemplazar código `unsafe`
- `3` — Obtener sugerencias de código seguro
- `4` — Salir

## 4) Probar con ejemplos incluidos

```bash
cargo run --manifest-path rmorph/Cargo.toml
```

Luego elige la opción `1` e ingresa una ruta como:

```text
rmorph/examples
```

o un archivo concreto como:

```text
rmorph/examples/assign_to_deref/simple.rs
```

## 5) Salida esperada

La extracción genera carpetas como `result/` y, en algunos flujos, `result_changed/` según la opción elegida.

## Notas
- Los scripts legacy en `code_src/` quedan como referencia histórica; el flujo principal y mantenido es `rmorph/`.
- Para una compilación optimizada:

```bash
cargo build --release --manifest-path rmorph/Cargo.toml
```
