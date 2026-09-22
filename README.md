# Herramienta para detección y modificación de código unsafe en Rust

Este repositorio incluye el flujo principal de análisis y modificación de bloques `unsafe` en Rust, implementado en `rmorph/`.

## Objetivo

- Detectar bloques `unsafe` en archivos o directorios Rust.
- Extraer esos bloques a artefactos de salida.
- Identificar patrones relevantes dentro del código `unsafe`.
- Aplicar reemplazos automáticos cuando corresponda.
- Generar sugerencias de código seguro.

## Alcance

Este repositorio mantiene el flujo principal en Rust (`rmorph/`). Los scripts legacy en `code_src/` se conservan solo por compatibilidad histórica, pero no son la ruta recomendada para uso actual.

## Requisitos previos

- Rust (`rustc` y `cargo`) instalado y en `PATH`.
- Permisos de lectura y escritura sobre la carpeta que se va a analizar.
- Opcional: Python 3 si se desea revisar los scripts heredados en `code_src/`.

## Compilar y ejecutar

Desde la raíz del repositorio:

```bash
cargo build --manifest-path rmorph/Cargo.toml
cargo run --manifest-path rmorph/Cargo.toml
```

También puede compilar la versión release:

```bash
cargo build --release --manifest-path rmorph/Cargo.toml
cargo run --release --manifest-path rmorph/Cargo.toml
```

## Menú interactivo

El programa principal es interactivo y ofrece estas opciones:

- `1` — Extraer código `unsafe`
- `2` — Reemplazar código `unsafe`
- `3` — Obtener sugerencias de código seguro
- `4` — Salir

Cuando elija una acción, se le pedirá la ruta del archivo o directorio Rust a analizar.

### Ejemplo rápido

```bash
cargo run --manifest-path rmorph/Cargo.toml
```

Luego:

1. Seleccione `1`.
2. Ingrese una ruta de prueba, por ejemplo:

```text
rmorph/examples
```

o un archivo puntual:

```text
rmorph/examples/assign_to_deref/simple.rs
```

## Estructura principal

- `rmorph/src/main.rs` — menú principal y flujo de ejecución.
- `rmorph/src/extractor.rs` — extracción de bloques `unsafe`.
- `rmorph/src/pattern_detector.rs` — detección de patrones.
- `rmorph/src/modifier.rs` — reemplazos automáticos.
- `rmorph/src/suggestions.rs` — sugerencias de código seguro.

## Salida generada

Por defecto, la extracción crea carpetas de resultados bajo la raíz del proyecto, como `result/` y, según la opción, `result_changed/`, pero esto tras modificar.

## Documento final del proyecto

El informe final y otros documentos técnicos del proyecto, tales como diagramas y referencias analizadas se encuentran en la carpeta `Documentacion/`.

## Notas importantes

- Las rutas y comandos documentados en este README corresponden al flujo real de `rmorph`.
- Si se quiere analizar un proyecto grande, conviene probar primero sobre un subdirectorio o archivo pequeño.

## Problemas frecuentes

- `cargo` no está en `PATH`: instalar Rust desde https://rustup.rs/.
- Ruta inválida: introducir la ruta completa o relativa correcta al archivo/directorio.
- Error de parseo: revisar si el archivo Rust usa sintaxis poco habitual o macros complejas.
- Error en ejecución: revisar que el proyecto compila antes de aplicarle la herramineta, en caso contrario no se asegura que pueda ser de alguna utilidad.

## Contacto

- Autor: Uriel Gz — https://github.com/Uriel-Gz

