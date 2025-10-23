# script.py
import os
import pattern
import u_cases.u_analizer as analizer

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
                            type_uc = analizer.analyze(subcode)
                            if type_uc == None:
                                i += 1
                                subcode += code[i]
                            elif type_uc != None:
                                new_code += type_uc.replace(subcode)
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

            # Crea los directorios necesarios en 'result' si no existen
            os.makedirs(f'result/{str(root)[2:]}', exist_ok=True)

            # Escribe el nuevo contenido de vuelta al archivo
            with open(f'result/{finalFileName}', 'w') as file:
                file.write(new_code)

replace_unsafe_code('./x')