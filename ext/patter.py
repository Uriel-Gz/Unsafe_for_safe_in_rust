import re

texto = "" \
"unsafe {" \
"       let *ptr = ssl.a(1,2,3)" \
"}"

patron = r'let \*(\w+) =\s*([a-zA-Z0-9_.(),]+)'

coincidencia = re.search(patron, texto)

if coincidencia:
    var = coincidencia.group(1)  # Captura el nombre
    asig = coincidencia.group(2)    # Captura el color
    print(f'mem::transmute({var},{asig})')