Ejemplos de uso de `syn` para localizar y reemplazar `unsafe` por `unp_reemplazado`.

Instrucciones rápidas:

- Abrir una terminal (bash) en `code_src/syn_examples`.
- Ejecutar `cargo run --release`.
- El programa leerá todos los `.rs` dentro de la carpeta `examples/` y escribirá las versiones modificadas en `out/`.

Extracción batch de bloques `unsafe`:

- Además de escribir la versión modificada de cada fichero en `out/`, el programa ahora extrae todos los bloques `unsafe { ... }` encontrados y los guarda como archivos individuales bajo `out/unsafe_blocks/<original_stem>/`.
- Cada bloque se guarda en un archivo llamado `<original_stem>_unsafe_<n>.rs` (donde `n` es un índice empezando en 1).

Ejemplo de estructura generada:

```
out/
	your_example.rs        # versión modificada donde `unsafe` fue reemplazado por `unp_reemplazado`
	unsafe_blocks/
		your_example/
			your_example_unsafe_1.rs
			your_example_unsafe_2.rs
```

Notas:

- El bloque extraído se escribe tal cual con los tokens (`unsafe { ... }`). Si necesitas archivos `rs` válidos por separado puedes envolverlos manualmente (por ejemplo, en una función) o modificar el extractor.
- Requiere `cargo` y toolchain Rust instalados.
- Si quieres probar manualmente sobre un archivo, copia el `.rs` a `examples/` y vuelve a ejecutar.
