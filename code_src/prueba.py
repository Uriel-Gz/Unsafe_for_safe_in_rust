def extraer_bloques(texto, palabra_clave):
    bloques = []
    pos = 0
    while True:
        inicio = texto.find(f"{palabra_clave} {{", pos)
        if inicio == -1:
            break

        i = inicio + len(f"{palabra_clave} {{")
        start = i
        nivel = 1
        while i < len(texto):
            if texto[i] == '{':
                nivel += 1
            elif texto[i] == '}':
                nivel -= 1
                if nivel == 0:
                    bloques.append(texto[start:i])
                    pos = i + 1
                    break
            i += 1
        else:
            # No se cerró correctamente
            print("Error: bloque no cerrado correctamente.")
            break
    return bloques


with open('x/prueba.rs', 'r', encoding='utf-8') as f:
    contenido = f.read()

blk = extraer_bloques(contenido, 'unsafe')
print(blk)