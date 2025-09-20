import os
import json
from bs4 import BeautifulSoup
import re
from datetime import datetime

def create_json_visualizer_dir(base_dir):
    """Crea el directorio json_visualizer si no existe y retorna su ruta."""
    json_dir = os.path.join(base_dir, "json_visualizer")
    if not os.path.exists(json_dir):
        os.makedirs(json_dir)
        print(f"Directorio creado: {json_dir}")
    return json_dir

def extract_code_blocks(html_content):
    """Extrae los bloques de código unsafe y su información del HTML."""
    soup = BeautifulSoup(html_content, 'html.parser')
    repository = None
    current_file = None
    files_data = {}
    
    # Buscar el nombre del repositorio
    h3_tags = soup.find_all('h3')
    if h3_tags:
        first_h3 = h3_tags[0].text
        match = re.search(r'In the repository \(subfolder/s\) ([^/]+)', first_h3)
        if match:
            repository = match.group(1)
    
    # Iterar sobre todos los elementos para encontrar archivos y bloques de código
    for elem in soup.find_all(['a', 'code']):
        if elem.name == 'a':
            # Es un enlace que contiene el nombre del archivo
            file_path = elem.em.text if elem.em else None
            if file_path:
                current_file = file_path
                if current_file not in files_data:
                    files_data[current_file] = []
        
        elif elem.name == 'code' and elem.get('class') and 'rust' in elem.get('class'):
            # Es un bloque de código
            if not elem.get('id') and current_file:  # Solo los bloques principales, no los expandidos
                # Buscar el número de línea en el ID del siguiente bloque
                next_block = elem.find_next('code')
                line_number = None
                if next_block and next_block.get('id'):
                    line_match = re.search(r'-(\d+)$', next_block.get('id'))
                    if line_match:
                        line_number = int(line_match.group(1))
                
                code_text = elem.text.strip()
                if code_text.startswith('unsafe'):
                    files_data[current_file].append({
                        "linea": line_number,
                        "codigo": code_text
                    })
    
    return repository, files_data

def create_summary_file(json_dir, processed_files):
    """Crea un archivo de resumen con la información de los archivos procesados."""
    summary = {
        "fecha_procesamiento": datetime.now().strftime("%Y-%m-%d %H:%M:%S"),
        "archivos_procesados": processed_files
    }
    
    summary_path = os.path.join(json_dir, "summary.json")
    with open(summary_path, 'w', encoding='utf-8') as file:
        json.dump(summary, file, indent=2, ensure_ascii=False)
    
    print(f"Archivo de resumen creado: {summary_path}")

def process_html_files(directory):
    """Procesa todos los archivos HTML en el directorio y genera archivos JSON."""
    # Crear directorio para los JSON
    json_dir = create_json_visualizer_dir(directory)
    processed_files = []
    
    for filename in os.listdir(directory):
        if filename.endswith(('.html', '.htm')):
            html_path = os.path.join(directory, filename)
            json_filename = f"{os.path.splitext(filename)[0]}.json"
            json_path = os.path.join(json_dir, json_filename)
            
            print(f"Procesando: {filename}")
            
            try:
                # Leer el archivo HTML
                with open(html_path, 'r', encoding='utf-8') as file:
                    html_content = file.read()
                
                # Extraer la información
                repository, files_data = extract_code_blocks(html_content)
                
                if repository and files_data:
                    # Crear la estructura JSON
                    json_data = {
                        "repositorio": repository,
                        "archivos": [
                            {
                                "ruta": file_path,
                                "ocurrencias": occurrences
                            }
                            for file_path, occurrences in files_data.items()
                            if occurrences  # Solo incluir archivos con ocurrencias
                        ],
                        "total_ocurrencias": sum(len(occurrences) for occurrences in files_data.values())
                    }
                    
                    # Guardar el archivo JSON
                    with open(json_path, 'w', encoding='utf-8') as file:
                        json.dump(json_data, file, indent=2, ensure_ascii=False)
                    
                    processed_files.append({
                        "nombre_archivo": filename,
                        "repositorio": repository,
                        "total_ocurrencias": json_data["total_ocurrencias"],
                        "archivos_afectados": len(json_data["archivos"])
                    })
                    
                    print(f"Archivo JSON creado: {json_path}")
                else:
                    print(f"No se encontró información válida en {filename}")
            
            except Exception as e:
                print(f"Error procesando {filename}: {str(e)}")
    
    # Crear archivo de resumen
    if processed_files:
        create_summary_file(json_dir, processed_files)
    
    return len(processed_files)

def main():
    while True:
        directory = input("Ingrese la ruta del directorio con los archivos HTML (o 'q' para salir): ")
        
        if directory.lower() == 'q':
            break
        
        if not os.path.isdir(directory):
            print("La ruta ingresada no es un directorio válido.")
            continue
        
        num_processed = process_html_files(directory)
        print(f"\nProcesamiento completado. Se procesaron {num_processed} archivos HTML.")
        print(f"Los archivos JSON se guardaron en la carpeta 'json_visualizer'")

if __name__ == "__main__":
    main()