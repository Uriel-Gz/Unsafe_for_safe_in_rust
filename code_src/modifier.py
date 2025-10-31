# script.py
import os
from u_cases.u_analizer import UnsafeAnalyzer
from lexer import Lexer

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
                
                #* reconstruyo el codigo leido por lineas y lo tokenizo
                code = '\n'.join(code)
                lexer = Lexer(code)
                tokens = lexer.tokenize()

                pos = 0
                while pos < len(tokens):

                    if tokens[pos].type == 'UNSAFFE':
                        #* Salta 'unsafe' y '{'
                        pos += 2
                        subcode = []
                        analizer = UnsafeAnalyzer()
                        
                        while pos < len(code):
                            subcode.append(tokens[pos])
                            type_uc = analizer.analyze(subcode)

                            if type_uc == None:
                                pos += 1
                                if pos < len(tokens):
                                    subcode.append(tokens[pos])
                            else:
                                type_u = type_uc['type_u'] if type_uc != None else None
                                methadata = type_uc['matched_data'] if type_uc != None else None
                                code = type_u.replace(type_u, code, methadata)
                                break
                    else:
                        # new_code += subcode
                        pos += 1
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