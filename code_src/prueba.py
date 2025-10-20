def extraer_bloques(texto, palabra_clave):
    import re
    bloques = []
    # Buscar "palabra_clave" seguida opcionalmente de espacios y "{"
    patron = re.compile(rf'{re.escape(palabra_clave)}\s*\{{')
    for match in patron.finditer(texto):
        inicio = match.end()  # Posición después de "{"
        stack = [inicio]  # Usar stack para manejar anidamiento
        i = inicio
        while i < len(texto) and stack:
            if texto[i] == '{':
                stack.append(i)
            elif texto[i] == '}':
                stack.pop()
                if not stack:
                    # Bloque cerrado, extraer desde inicio hasta i (excluyendo la llave de cierre)
                    bloques.append(texto[inicio:i])
                    break
            i += 1
        if stack:
            print(f"Error: bloque no cerrado correctamente para '{palabra_clave}' en posición {match.start()}.")
    return bloques


with open('x/prueba.rs', 'r', encoding='utf-8') as f:
    contenido = f.read()

blk = extraer_bloques(contenido, 'unsafe')
print(blk)