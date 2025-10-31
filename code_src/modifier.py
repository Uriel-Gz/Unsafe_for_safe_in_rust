# script.py
import os
from u_cases.u_analizer import UnsafeAnalyzer

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
                        
                        while i < len(code):
                            analizer = UnsafeAnalyzer()
                            type_uc = analizer.analyze(subcode)
                            if type_uc == None:
                                if 'unsafe {' not in code[i]:
                                    subcode += code[i]
                                i += 1
                            else:
                                type_u = type_uc['type_u'] if type_uc != None else None
                                methadata = type_uc['matched_data'] if type_uc != None else None
                                new_code += type_u.replace(type_u, subcode, methadata)
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

if __name__ == "__main__":
    replace_unsafe_code('./unsafe-code-examples')