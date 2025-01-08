
import os
import re
import fnmatch

def indentify_unsafe_code_in_directory(directory):
    # Contador para bloques unsafe
    total_count = 0

    file = open('unsafe_code.txt', 'w')
    # Recorre el directorio y sus subdirectorios
    for root, dirs, files in os.walk(directory):
        for filename in fnmatch.filter(files, '*.rs'):
            file_path = os.path.join(root, filename)

            # Lee el contenido del archivo
            with open(file_path, 'r', encoding='utf-8') as f:
                code = f.read()
            # Reemplaza la sección unsafe por otro código
            # inside_unsafe = False
            # count = 0

            pattern = r'unsafe\s*\{(?:[^{}]|\{(?:[^{}]|\{[^{}]*\})*\})*\}'
            matches = re.findall(pattern, code)


            count = len(matches)
            total_count += count

            for match in matches:
                file.write(f'En archivo {file_path}: \n') 
                file.write(f'{match} \n \n')


# Llama a la función con la ruta del directorio
indentify_unsafe_code_in_directory('../proyectos_para_analisar')