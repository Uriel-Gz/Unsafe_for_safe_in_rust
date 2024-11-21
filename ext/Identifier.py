
import os
import fnmatch

def indentify_unsafe_code_in_directory(directory):
    # Contador para bloques unsafe
    total_count = 0

    # Recorre el directorio y sus subdirectorios
    for root, dirs, files in os.walk(directory):
        for filename in fnmatch.filter(files, '*.rs'):
            file_path = os.path.join(root, filename)
            print(f'Procesando archivo: {file_path}')
            
            # Lee el contenido del archivo
            with open(file_path, 'r') as file:
                code = file.readlines()

            # Reemplaza la sección unsafe por otro código
            inside_unsafe = False
            count = 0

            for line in code:
                if 'unsafe {' in line:
                    inside_unsafe = True
                    count += 1
                    continue
                if inside_unsafe and '}' in line:
                    inside_unsafe = False
                    continue

            total_count += count

    print(f'Cantidad total de bloques unsafe en los archivos .rs: {total_count}')

# Llama a la función con la ruta del directorio
indentify_unsafe_code_in_directory('../proyectos_para_analisar')