# script.py

def replace_unsafe_code(origin):
    # Lee el contenido del archivo
    with open(origin, 'r') as file:
        code = file.readlines()

    # Reemplaza la sección unsafe por otro código
    new_code = []
    inside_unsafe = False

    for line in code:
        if 'unsafe {' in line:
            inside_unsafe = True

            # new_code.append(cambio)
            continue
        if inside_unsafe and '*ptr' in line:
            line = line.replace('*ptr', 'replace',10)
            new_code.append(line)
            continue
        if inside_unsafe and '}' in line:
            inside_unsafe = False
            continue
        else:
            new_code.append(line)

    # Escribe el nuevo contenido de vuelta al archivo
    with open(origin, 'w') as file:
        file.writelines(new_code)

# Llama a la función con la ruta del archivo Rust
replace_unsafe_code('prueba.rs')