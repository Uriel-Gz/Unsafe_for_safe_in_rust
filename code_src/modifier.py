# script.py
import os
from u_cases.u_analizer import UnsafeAnalyzer
from lexer import Lexer


def ub_delete(string):
    lex = Lexer(string)
    tokens = lex.tokenize()
    new_code = []
    pos = 0
    while pos < len(tokens):
        if tokens[pos].value == 'unsafe':
            u_open = 1

            #* Salta 'unsafe' y '{'
            pos += 2
            
            while pos < len(tokens):
                if tokens[pos].value == '{':
                    u_open += 1
                    new_code.append(tokens[pos])
                elif tokens[pos].value == '}' and u_open-1 == 0:
                    pos += 1
                    u_open -= 1
                    break
                elif tokens[pos].value == '}' and u_open-1 > 0:
                    u_open -= 1
                    new_code.append(tokens[pos])
                else:
                    new_code.append(tokens[pos])
                pos += 1
        else:
            new_code.append(tokens[pos])
            pos += 1
    
    result_code = ''
    space = ''

    for item in new_code:
        if item.type in ['COMMENT', 'RBRACE', 'LBRACE', 'SEMICOLON']:
            if item.type == 'LBRACE':
                space += '    '
            elif item.type == 'RBRACE':
                space = space[:-4]
                result_code = result_code[:-4]
            if item.type == 'SEMICOLON' or item.type == 'LBRACE':
                result_code += ' ' + item.value + '\n'
            else:
                result_code += item.value + '\n'

            result_code += space
        elif item.type in ['LPAREN', 'RPAREN', 'COMMA', 'DOT', 'STRING_LITERAL']:
            result_code += item.value
        else:
            if result_code and result_code[-1] not in [' ', '\n', '.', '(', '{', ';']:
                result_code += ' '
            result_code += item.value

    return result_code
    



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
                code = ''.join(code)
                lexer = Lexer(code)
                tokens = lexer.tokenize()

                pos = 0
                while pos < len(tokens):

                    if tokens[pos].value == 'unsafe':
                        u_open = 1

                        #* Salta 'unsafe' y '{'
                        pos += 2
                        subcode = []
                        analizer = UnsafeAnalyzer()
                        subcode.append(tokens[pos])
                        
                        while pos < len(tokens):
                            if tokens[pos].value == '{':
                                u_open += 1
                            elif tokens[pos].value == '}' and u_open-1 == 0:
                                u_open -= 1
                                break
                            elif tokens[pos].value == '}' and u_open-1 > 0:
                                u_open -= 1

                            type_uc = analizer.analyze(subcode)

                            if type_uc == None:
                                pos += 1
                                if pos < len(tokens):
                                    subcode.append(tokens[pos])
                            else:
                                type_u = type_uc['type_u'] if type_uc != None else None
                                methadata = type_uc['matched_data'] if type_uc != None else None
                                
                                code = type_u.replace(type_u, code, methadata)
                                new_code = code
                                subcode = []
                    else:
                        new_code = code
                        pos += 1
                    new_code = ub_delete(new_code)
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