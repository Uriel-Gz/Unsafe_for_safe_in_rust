
# Herramienta para detección y modificación de código unsafe en Rust

Este repositorio agrupa utilidades para identificar, extraer, analizar y —opcionalmente— modificar fragmentos de código `unsafe` en proyectos Rust.

> Nota importante: El flujo principal y mantenido ahora es el programa Rust ubicado en `rmorph/`.
> Los scripts en `code_src/` (Python) permanecen en el repositorio por compatibilidad/registro, pero están obsoletos: use preferentemente `rmorph/`.

**Objetivo y alcance**
- Objetivo: ofrecer un flujo reproducible y robusto para detectar bloques `unsafe`, extraerlos a archivos independientes, clasificar patrones, generar HTML/JSON de resultado y aplicar transformaciones basadas en plantillas.
- Alcance: este documento describe cómo compilar y ejecutar el programa Rust (`rmorph/`), la salida que genera, ejemplos y cómo personalizar la detección y los reemplazos.

**Requisitos previos**
- Hardware: PC con al menos 2 CPU y 4 GB de RAM (recomendado 4+ CPU y 8+ GB para proyectos grandes).
- Software:
  - Rust toolchain (`rustc`, `cargo`) instalado y en PATH.
  - (Opcional) Python 3.7+ si desea inspeccionar o ejecutar los scripts legacy en `code_src/`.
  - Conexión a Internet para descargar dependencias al compilar (solo la primera vez).
- Permisos: acceso de lectura/escritura a los directorios de los proyectos y permiso para crear carpetas de salida (`result/`, `result_changed/`, etc.).

---

## Cómo funciona el programa Rust (`rmorph`)

Resumen rápido:
- `rmorph` es una crate Rust que usa `syn` para parsear archivos `*.rs`, recorre el AST buscando bloques `unsafe` y patrones relevantes, y genera:
  - archivos con cada bloque `unsafe` extraído,
  - metadatos JSON de cada bloque,
  - reportes JSON agregados y HTML interactivo para revisión,
  - (opcional) un conjunto de archivos modificados con sustituciones seguras en `result_changed/`.

Componentes principales (ubicados en `rmorph/src/`):
- `extractor.rs`: recorre directorios, extrae bloques `unsafe`, genera los ficheros `unsafe_blocks` y `unsafe_ast`, invoca `pattern_detector` y genera HTML/JSON.
- `pattern_detector.rs`: detecta patrones específicos dentro de bloques `unsafe` (p. ej. `deref_expr`, `assign_to_deref`, `raw_addr_expr`, `matching_call_omission`, `mutable_ref_expr`, `unsafe_block`).
- `modifier.rs`: aplica transformaciones basadas en plantillas (templates) para patrones detectados y escribe resultados en `result_changed/`.
- `extractor_utils.rs`: helpers para generar resúmenes y agregados.

Salida principal generada por `rmorph` (por defecto en `result/`):
- `result/unsafe_blocks/...` — archivos `*_unsafe_#.rs` con cada bloque extraído.
- `result/unsafe_ast/...` — metadatos JSON por bloque (`*_unsafe_#.meta.json`).
- `result/patterns/` — archivos `*_patterns.json` con patrones detectados por fichero.
- `result/html_patterns/...` — HTML con visualización interactiva por fichero.
- `result/report/summary.json` — resumen agregado (si el usuario lo solicita durante la extracción).
- `result_changed/` — (cuando aplica `replace`) árbol de directorios con los archivos modificados.

### Construir y ejecutar

Desde la raíz del repositorio, puede compilar y ejecutar con `cargo` (ejemplos):

Linux / macOS / WSL:

```bash
# Compilar en modo debug
cargo build --manifest-path rmorph/Cargo.toml

# Ejecutar en modo interactivo (mostrará un menú)
cargo run --manifest-path rmorph/Cargo.toml

# Compilar y ejecutar la versión optimizada (release)
cargo build --release --manifest-path rmorph/Cargo.toml
cargo run --release --manifest-path rmorph/Cargo.toml
```

Windows (PowerShell):

```powershell
# Construir
cargo build --manifest-path rmorph/Cargo.toml

# Ejecutar
cargo run --manifest-path rmorph/Cargo.toml

# Ejecutar el binario generado directamente (ejemplo debug)
.\rmorph\target\debug\rmorph.exe
```

El programa es interactivo: al ejecutarlo verá un menú con opciones (Extraer, Reemplazar, Obtener sugerencias, Salir). Para cada acción se le pedirá la ruta del archivo o directorio Rust a procesar.

Ejemplo corto (extracción):

1. `cargo run --manifest-path rmorph/Cargo.toml`
2. Seleccione `1` (Extraer código unsafe)
3. Ingrese la ruta del proyecto a analizar, por ejemplo `mi-proyecto`

Resultados: al finalizar habrá archivos en `result/` tal como se describió arriba.

---

## Procedimiento detallado (paso a paso con ejemplos)

1) Preparar proyecto a analizar
- Coloque los repositorios Rust dentro de `<path/>` o tenga la ruta absoluta lista.
- Puede usar los ejemplos de prueba en `rmorph/examples/` para pruebas rápidas.

2) Ejecutar extracción de bloques `unsafe`

```bash
cargo run --manifest-path rmorph/Cargo.toml
# elegir opción 1
# ingresar: projects_to_review/mi-proyecto
```

3) (Opcional) Generar resumen
- Al finalizar la extracción el programa pregunta si quiere crear un resumen por tipo de patrón. Responda `s` para generar `result/report/summary.json`.

4) Aplicar transformaciones automáticas (experimental)

```bash
cargo run --manifest-path rmorph/Cargo.toml
# elegir opción 2
# ingresar: projects_to_review/mi-proyecto
```

- Los archivos modificados se escribirán en `result_changed/`. Haga un respaldo antes de aplicar cambios a proyectos reales.

5) Obtener sugerencias de código seguro
- Desde el menú, elija la opción 3 y ingrese la ruta del proyecto o archivo.

---

## Configuraciones avanzadas / trucos
- Personalizar patrones: edite `rmorph/src/pattern_detector.rs` para añadir o ajustar la detección de patrones.
- Plantillas de reemplazo: `rmorph/src/modifier.rs` contiene un `TemplateManager` con plantillas por tipo de patrón; edítelo para adaptar las sustituciones.
- Probar con ejemplos: `rmorph/examples/` contiene pequeños snippets para probar las detecciones y las transformaciones.
- Integración CI: para integración continua, ejecute `cargo run --manifest-path rmorph/Cargo.toml` en un job y archive `result/` como artefacto.
- Automatización: el programa es interactivo; si necesita automatizar, considere modificar `rmorph/src/main.rs` para aceptar argumentos CLI o usar `expect`/scripts que envíen la entrada.

## Patrones detectados (ejemplos)
- `deref_expr` — dereferencias (`*ptr`, `*(expr)`)
- `assign_to_deref` — asignaciones a referencias/dereferencias (`*p = x`)
- `raw_addr_expr` — direcciones raw
- `matching_call_omission` — llamadas dentro de `unsafe` que omiten comprobaciones (por ejemplo `Some(...)`)
- `mutable_ref_expr` — referencias mutables dentro de `unsafe`
- `unsafe_block` — bloque `unsafe` genérico cuando no se detectan patrones más específicos

---

## Solución de problemas frecuentes
- `cargo` no encontrado: instale Rust desde https://rustup.rs/ y asegúrese que `cargo` está en `PATH`.
- Errores de parseo (archivos saltados): algunos archivos Rust con sintaxis no estándar o macros complejas pueden producir errores de parseo por `syn`; revise la salida y ejecute sobre archivos individuales si es necesario.
- Permisos de escritura: ejecute el binario desde una carpeta donde tenga permisos de creación (`result/`).
- Reemplazos que fallan: las transformaciones automáticas son experimentales; revise `rmorph/src/modifier.rs` y ejecute en copias de prueba.

---

## Migración desde los scripts Python (nota)
- Los scripts en `code_src/` correspondían a versiones anteriores de extracción y conversión. Actualmente la versión recomendable y mantenida es `rmorph/` (Rust).
- Si necesita ejecutar los scripts Python por compatibilidad, hágalo en un entorno virtual y con las dependencias indicadas, pero evite usarlos para análisis a gran escala en favor de `rmorph`.

---

**Contacto / Soporte**
- Autor: Uriel Gz — https://github.com/Uriel-Gz
- Para issues: cree un issue en el repositorio con descripción y pasos para reproducir.

**Versión del documento**
- Versión: 2026-06-03 (actualizado: integra `rmorph` como flujo principal)

