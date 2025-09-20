# Unsafe Code Analysis Tools for Rust

Este repositorio contiene un conjunto de herramientas para analizar y procesar código unsafe en proyectos Rust. Las herramientas están diseñadas para ayudar en la identificación, extracción y análisis de patrones de uso de bloques unsafe en repositorios Rust.

## Estructura Adoptada para el Proyecto
```
.
├── code_src/                     # Directorio de código fuente
│   ├── extractor.py              # Extractor de código unsafe
│   ├── html_to_json_converter.py # Conversor de HTML a JSON
│   └── modifier.py               # Modificador de código
├── Documentacion/                # Documentación del proyecto
└── structures/                   # Estructuras de datos y pruebas
```

## Requisitos Previos

- Python 3.7 o superior
- BeautifulSoup4 (para procesamiento HTML)
- Rust (última versión estable)

Para instalar las dependencias de Python:

```bash
pip install beautifulsoup4
```

## Herramientas Disponibles

### 1. HTML to JSON Converter

Esta herramienta convierte archivos HTML que contienen análisis de código unsafe en archivos JSON estructurados.

#### Uso

```bash
python html_to_json_converter.py
```

La herramienta solicitará la ruta del directorio que contiene los archivos HTML a procesar. Los archivos JSON resultantes se guardarán en una subcarpeta `json_visualizer`.

#### Estructura del JSON generado

```json
{
  "repositorio": "nombre-repo",
  "archivos": [
    {
      "ruta": "path/to/file.rs",
      "ocurrencias": [
        {
          "linea": 123,
          "codigo": "unsafe { ... }"
        }
      ]
    }
  ],
  "total_ocurrencias": 42
}
```

También se genera un archivo `summary.json` con estadísticas globales del procesamiento.

### 2. Extractor de Código Unsafe

Herramienta para extraer bloques de código unsafe directamente de repositorios Rust.

#### Uso

```bash
python extractor.py <ruta-repositorio>
```

### 3. Modificador de Código

Utilidad para modificar y refactorizar código unsafe en proyectos Rust.

#### Uso

```bash
python modifier.py <ruta-repositorio>
```

## Flujo de Trabajo Recomendado

1. **Preparación**: 
   - Colocar los archivos o directorios de código Rust en la carpeta `projects_to_review`
   - Asegurarse de que todas las dependencias están instaladas

2. **Extracción de Código Unsafe**: 
   ```bash
   cd code_src
   python extractor.py
   ```
   Esto analizará los proyectos en `projects_to_review` y generará archivos HTML con los resultados.

3. **Conversión a JSON**: 
   ```bash
   python html_to_json_converter.py
   ```
   Convertir los archivos HTML generados a formato JSON estructurado.

4. **Análisis de Patrones**: 
   ```bash
   python patter.py
   ```
   Identificar y analizar patrones en el código unsafe encontrado.

5. **Modificación (opcional)**: 
   ```bash
   python modifier.py
   ```
   Aplicar modificaciones o refactorizaciones si es necesario.

## Estructura de los Resultados (aquellas que son necesaria para usar la herramienta *)


- `projects_to_review/`: Carpeta donde se colocan los proyectos a analizar  *
- `reviewed_projects/`: Contiene los proyectos ya analizados                *
- `results/`: Contiene los archivos/proyectos modificados por la herramienta modifier.py.
- `Documentacion/repositories_analised/`: Archivos HTML con los resultados del análisis, dichos archivos pueden mostrar ms en profundidad el código de donde se sacó el fragmento correspondiente, esto se hace haciendo click en el nombre del archivo que aparece en la visualización.
- `Documentacion/repositories_analised/json_visualizer/`: Archivos JSON procesados, utiles para tener conocimenito puntual de los resultdos, no asi todo el código del mismo.

## Contribuir

Las contribuciones son bienvenidas. Por favor, sigue estos pasos:

1. Fork el repositorio
2. Crea una rama para tu feature (`git checkout -b feature/AmazingFeature`)
3. Commit tus cambios (`git commit -m 'Add some AmazingFeature'`)
4. Push a la rama (`git push origin feature/AmazingFeature`)
5. Abre un Pull Request

## Licencia

Este proyecto está bajo la Licencia MIT - ver el archivo [LICENSE](LICENSE) para más detalles.

## Contacto

Uriel Gz - [Github](https://github.com/Uriel-Gz)

Link del proyecto: [https://github.com/Uriel-Gz/Unsafe_for_safe_in_rust](https://github.com/Uriel-Gz/Unsafe_for_safe_in_rust)

## Reconocimientos

* [Rust Programming Language](https://www.rust-lang.org/)
* [Beautiful Soup Documentation](https://www.crummy.com/software/BeautifulSoup/bs4/doc/)
* Todos los mantenedores de repositorios analizados
