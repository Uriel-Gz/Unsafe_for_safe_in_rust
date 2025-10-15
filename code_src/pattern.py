import re


# posibles patrones que se pueden dar
patterns = {
    'ptr_asign': r'unsafe\s*\{(.*?)\*([a-zA-Z_][a-zA-Z0-9_]*)\s*=\s*(.+?);([^}])*\}',
    'fnc': r'([a-zA-Z_][a-zA-Z0-9_]*)\s*\(([^)]*)\)',
    'method': r'(\w+)\.([a-zA-Z_][a-zA-Z0-9_]*)\s*\(([^)]*)\)',
    'struct': r'struct\s+([A-Z][a-zA-Z0-9_]*)\s*{([^}]*)}',
    'enum': r'enum\s+([A-Z][a-zA-Z0-9_]*)\s*{([^}]*)}',
    'impl': r'impl\s+([A-Z][a-zA-Z0-9_]*)\s*{([^}]*)}',
    'trait': r'trait\s+([A-Z][a-zA-Z0-9_]*)\s*{([^}]*)}',
    'default': r'unsafe\s*{([^}]*)}',
}

# posibles reemplazos para los patrones descriptos
replaces = {
    'ptr_asign': lambda before, var, value, after: f'{before} mem::replace({var}, {value}); {after}',
    'fnc': lambda name, args: f'{name}({args})',
    'method': lambda obj, name, args: f'{obj}.{name}({args})',
    'struct': lambda name, fields: f'struct {name} {{ {fields} }}',
    'enum': lambda name, variants: f'enum {name} {{ {variants} }}',
    'impl': lambda name, body: f'impl {name} {{ {body}}}',
    'trait': lambda name, body: f'trait {name} {{ {body}}}',
    'default': lambda body: f'{{ {body} }}',
}



def identify_unsafe(texto):
    for clave, patron in patterns.items():
        coincidencia = re.search(patron, texto, re.DOTALL)
        if coincidencia:
            return clave, coincidencia.groups()

    return None, None

def replace_pattern(texto, tipo, grupo):
    if tipo and tipo in replaces:
        if tipo == 'ptr_asign':
            texto = texto.replace(texto, replaces[tipo](*grupo))
        if tipo == 'fnc':
            pass
        if tipo == 'method':
            pass
        if tipo == 'struct':
            pass
        if tipo == 'enum':
            pass
        if tipo == 'impl':
            pass

        if tipo == 'trait':
            pass

        if tipo == 'default'        :
            pass

    return texto


def pp(a):

# Diccionario de patrones parametrizados
    pattern = r'unsafe {\n?(.*?)\*([a-zA-Z_][a-zA-Z0-9_]*)\s*=\s*(.+?);(.*?)}'

    match = re.search(pattern, a)
    if match:
        group = match.groups()
        print(group)

sr = """    unsafe {        *ptr = 0;        println!("El valor del puntero es: {}", *ptr);    }"""
# pp(sr)

