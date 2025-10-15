# script.py
import os
import pattern

def replace_unsafe_code(origin):
    
    # Recorre el directorio y sus subdirectorios
    for root, _, files in os.walk(origin):

        for filename in files:
            # Reemplaza el codigo por uno que puede contener cambios
            new_code = ""

            file_path = os.path.join(root, filename)

            # Lee el contenido del archivo
            with open(file_path, 'r', encoding='utf-8') as f:
                code = f.readlines()

            if filename.endswith('.rs'):
                i = 0
                while i < len(code):
                    subcode = code[i]
                    if subcode.startswith('//') or subcode.startswith('/*') or subcode.startswith('*') or subcode.startswith('*/'):
                        new_code += subcode
                        continue
                    if 'unsafe {' in subcode:
                        
                        while True:
                            key , group = pattern.identify_unsafe(subcode)
                            if key == None:
                                i += 1
                                subcode += code[i]
                            elif key != 'default':
                                replaced_code = pattern.replace_pattern(subcode, key, group)
                                new_code += replaced_code
                                break
                            else:
                                new_code += subcode
                                break
                    else:
                        new_code += subcode
                    i += 1
            else: 
                new_code = '\n'.join(code)

            finalFileName = f'{str(root)[2:]}/{filename}'

            os.makedirs(f'result/{str(root)[2:]}', exist_ok=True)

            # Escribe el nuevo contenido de vuelta al archivo
            with open(f'result/{finalFileName}', 'w') as file:
                file.write(new_code)

replace_unsafe_code('./x')