# Gramáticas BNF para casos de código unsafe en Rust

# Caso 1: Asignaciones Unsafe Directas (ptr::write, ptr::read, Box::from_raw)
# stmt ::= "let" IDENTIFIER "=" expr ("." IDENTIFIER)? "(" expr? ")" ";"
#        | "let" IDENTIFIER "=" expr ("." IDENTIFIER)? "(" expr? ")" ";" "Some" "(" "ptr::read" "(" IDENTIFIER ")" ")" ";"
#        | "let" IDENTIFIER "=" expr ("." IDENTIFIER)? "(" expr? ")" ";" "Some" "(" "&*" IDENTIFIER ")" ";"
#        | "let" IDENTIFIER ":" "*" "mut" type "=" "Box::into_raw" "(" IDENTIFIER ")" ";" "Ok" "(" "Box::from_raw" "(" IDENTIFIER "as" "*" "mut" type ")" ")" ")"

# Caso 2: Manipulación de punteros (*, &mut, unsafe block)
# stmt ::= "*" "(" IDENTIFIER ")" "." IDENTIFIER "=" expr ";"
#        | "let" IDENTIFIER ":" "&mut" type "=" "&mut" "*" IDENTIFIER ";"
#        | "let" IDENTIFIER ":" "*" "mut" type "=" expr ";"
#        | "let" IDENTIFIER ":" "*" "mut" type "=" "*" "(" IDENTIFIER ")" "." IDENTIFIER ";"
#        | "*" "unsafe" "{" "&mut" "*" IDENTIFIER "}" ("." IDENTIFIER)* "=" expr ";"
#        | "let" IDENTIFIER "=" "&mut" "*" "(" IDENTIFIER "as" "*" "mut" type ")" ";"
#        | "*" "(" IDENTIFIER ")" "." IDENTIFIER "=" type "::" IDENTIFIER "(" "*" "(" IDENTIFIER ")" "." IDENTIFIER ")" ";"

# Caso 3: Manipulación de strings (*, +=)
# stmt ::= "*" IDENTIFIER "+=" STRING_LITERAL ";"

# Caso 4: Devolución de valores y asignación con Some
# stmt ::= "let" IDENTIFIER "=" IDENTIFIER "(" ")" ";"
#        | "(" "&" IDENTIFIER "." IDENTIFIER "," "&mut" IDENTIFIER "." IDENTIFIER "," "&" IDENTIFIER "." IDENTIFIER ")" ";"
#        | "*" "(" IDENTIFIER ")" "." IDENTIFIER "=" "Some" "(" IDENTIFIER ")" ";"
#        | "*" IDENTIFIER "=" "Some" "(" IDENTIFIER ")" ";"
#        | "let" IDENTIFIER "=" "*" IDENTIFIER ";"
#        | "let" IDENTIFIER ":" type "?=" "unsafe" "{" expr "}"
#        | "let" IDENTIFIER "=" "&" IDENTIFIER "as" "*" "const" type ("as" "*" "const" type)? ";" "*" IDENTIFIER ";"
#        | "unsafe" "{" expr "}"

# Caso 5: Llamada a funciones externas y uso de instrucciones ensamblador
# stmt ::= "extern" STRING_LITERAL "{" "fn" IDENTIFIER "(" (IDENTIFIER ":" type ("," IDENTIFIER ":" type)*)? ")" "->" type ";" "}"
#        | "core::arch::x86_64::_mm_storeu_ps" "(" IDENTIFIER ".as_mut().as_mut_ptr()" "as" "*" "mut" type "," expr ")" ";"
#        | "*" "(" IDENTIFIER ".as_mut().as_mut_ptr()" "as" "*" "mut" type ")" "=" expr ";"

# Caso 6: Casos especiales MaybeUninit y PhantomData con unsafe block
# stmt ::= "unsafe" "{" "let" ("mut")? IDENTIFIER "=" "MaybeUninit" "::" "<" type ">" "::uninit" "(" ")" ";" IDENTIFIER "(" expr? "," "std::ptr::null()" "," IDENTIFIER ".as_mut_ptr" "(" ")" ")" ";" IDENTIFIER ".assume_init" "(" ")" ";" "}"
#        | "unsafe" "{" "let" IDENTIFIER "=" type "::" "::data" "(" "self.ptr" ")" ";" IDENTIFIER ".as_ref()" ("." IDENTIFIER)* ".fetch_add" "(" NUMBER "," IDENTIFIER ")" ";" type "{" "ptr:" "self.ptr," "pd:" "PhantomData," "}" "}"

# Tokens comunes:
# IDENTIFIER ::= [a-zA-Z_][a-zA-Z0-9_]*
# expr ::= IDENTIFIER | NUMBER | STRING_LITERAL | expr OP expr | ...
# type ::= IDENTIFIER ("::" IDENTIFIER)* ("<" type ">")?
# OP ::= "+" | "-" | "*" | "/" | etc.

# Para simplificar, definiremos gramáticas más abstractas y parsers recursivos descendentes simples.

# Definiciones de gramáticas como diccionarios para facilitar el parsing

GRAMMARS = {
    'combined': {
        'start': 'stmt',
        'rules': {
            'stmt': [
                [],
                ['return_reference'],
                ['some', 'direct_assign', 'some'],
                ['some', 'pointer_manip', 'some']
            ],
        'some': [['stmt'],[]]
        }
    },



    'return_reference': {
        'start': 'stmt',
        'rules': {
            'stmt': [
                ['&', 'IDENTIFIER', '.', 'IDENTIFIER', ';'],
                ['&mut', 'IDENTIFIER', '.', 'IDENTIFIER', ';'],
                ['let', 'IDENTIFIER', '=', 'IDENTIFIER', '(', ')', ';'],
                ['(', '&', 'IDENTIFIER', '.', 'IDENTIFIER', ',', '&mut', 'IDENTIFIER', '.', 'IDENTIFIER', ',', '&', 'IDENTIFIER', '.', 'IDENTIFIER', ')', ';'],
                ['*', '(', 'IDENTIFIER', ')', '.', 'IDENTIFIER', '=', 'Some', '(', 'IDENTIFIER', ')', ';'],
                ['*', 'IDENTIFIER', '=', 'Some', '(', 'IDENTIFIER', ')', ';'],
                ['let', 'IDENTIFIER', '=', '*', 'IDENTIFIER', ';'],
                ['let', 'IDENTIFIER', ':', 'type', '?=', 'unsafe', '{', 'expr', '}'],
                ['let', 'IDENTIFIER', '=', '&', 'IDENTIFIER', 'as', '*', 'const', 'type', 'opt_as', ';', '*', 'IDENTIFIER', ';'],
                ['unsafe', '{', 'expr', '}']
            ],
            'opt_as': [['as', '*', 'const', 'type'], []],
            'expr': [['IDENTIFIER'], ['NUMBER'], ['STRING_LITERAL']],
            'type': [['IDENTIFIER']]
        }
    },


    'direct_assign': {
        'start': 'stmt',
        'rules': {
            'stmt': [
                ['opt_init','*','IDENTIFIER', '=', 'expr', ';'],
                ['let', 'IDENTIFIER', '=', 'expr', 'opt_method', '(', 'opt_expr', ')', ';'],
                ['let', 'IDENTIFIER', '=', 'Some', '(', 'expr', 'opt_method', '(', 'expr', ')', ')', ';'],
                ['let', 'IDENTIFIER', '=', 'Some', '(', '&', '*', 'IDENTIFIER', ')', ';'],
                ['let', 'IDENTIFIER', ':', '*', 'mut', 'type', '=', 'Box',':', ':', 'into_raw', '(', 'IDENTIFIER', ')', ';', 'Ok', '(', 'Box',':', ':', 'from_raw', '(', 'IDENTIFIER', 'as', '*', 'mut', 'type', ')', ')']
            ],
            'opt_init': [['let'], []],
            'opt_method': [[':',':' , 'IDENTIFIER'], []],
            'opt_expr': [['expr'], []],
            'expr': [['IDENTIFIER'], ['NUMBER'], ['STRING_LITERAL']],
            'type': [['IDENTIFIER']]
        }
    },


    'pointer_manip': {
        'start': 'stmt',
        'rules': {
            'stmt': [
                ['*', '(', 'IDENTIFIER', ')', '.', 'IDENTIFIER', '=', 'expr', ';'],
                ['let', 'IDENTIFIER', ':', '&mut', 'type', '=', '&mut', '*', 'IDENTIFIER', ';'],
                ['let', 'IDENTIFIER', ':', '*', 'mut', 'type', '=', 'expr', ';'],
                ['let', 'IDENTIFIER', ':', '*', 'mut', 'type', '=', '*', '(', 'IDENTIFIER', ')', '.', 'IDENTIFIER', ';'],
                ['*', 'unsafe', '{', '&mut', '*', 'IDENTIFIER', '}', 'opt_dots', '=', 'expr', ';'],
                ['let', 'IDENTIFIER', '=', '&mut', '*', '(', 'IDENTIFIER', 'as', '*', 'mut', 'type', ')', ';'],
                ['*', '(', 'IDENTIFIER', ')', '.', 'IDENTIFIER', '=', 'type', '::', 'IDENTIFIER', '(', '*', '(', 'IDENTIFIER', ')', '.', 'IDENTIFIER', ')', ';']
            ],
            'opt_dots': [['.', 'IDENTIFIER', 'opt_dots'], []],
            'expr': [['IDENTIFIER'], ['NUMBER'], ['STRING_LITERAL']],
            'type': [['IDENTIFIER']]
        }
    },
    'string_manip': {
        'start': 'stmt',
        'rules': {
            'stmt': [['*', 'IDENTIFIER', '+=', 'STRING_LITERAL', ';']]
        }
    },

    'extern_func': {
        'start': 'stmt',
        'rules': {
            'stmt': [
                ['extern', 'STRING_LITERAL', '{', 'fn', 'IDENTIFIER', '(', 'opt_params', ')', '->', 'type', ';', '}'],
                ['core::arch::x86_64::_mm_storeu_ps', '(', 'IDENTIFIER', '.as_mut().as_mut_ptr()', 'as', '*', 'mut', 'type', ',', 'expr', ')', ';'],
                ['*', '(', 'IDENTIFIER', '.as_mut().as_mut_ptr()', 'as', '*', 'mut', 'type', ')', '=', 'expr', ';']
            ],
            'opt_params': [['IDENTIFIER', ':', 'type', 'more_params'], []],
            'more_params': [[',', 'IDENTIFIER', ':', 'type', 'more_params'], []],
            'expr': [['IDENTIFIER'], ['NUMBER'], ['STRING_LITERAL']],
            'type': [['IDENTIFIER']]
        }
    },
    'special_cases': {
        'start': 'stmt',
        'rules': {
            'stmt': [
                ['unsafe', '{', 'let', 'opt_mut', 'IDENTIFIER', '=', 'MaybeUninit', '::', '<', 'type', '>', '::uninit', '(', ')', ';', 'IDENTIFIER', '(', 'opt_expr', ',', 'std::ptr::null()', ',', 'IDENTIFIER', '.as_mut_ptr', '(', ')', ')', ';', 'IDENTIFIER', '.assume_init', '(', ')', ';', '}'],
                ['unsafe', '{', 'let', 'IDENTIFIER', '=', 'type', '::', '::data', '(', 'self.ptr', ')', ';', 'IDENTIFIER', '.as_ref()', 'opt_dots', '.fetch_add', '(', 'NUMBER', ',', 'IDENTIFIER', ')', ';', 'type', '{', 'ptr:', 'self.ptr,', 'pd:', 'PhantomData,', '}', '}']
            ],
            'opt_mut': [['mut'], []],
            'opt_expr': [['expr'], []],
            'opt_dots': [['.', 'IDENTIFIER', 'opt_dots'], []],
            'expr': [['IDENTIFIER'], ['NUMBER'], ['STRING_LITERAL']],
            'type': [['IDENTIFIER']]
        }
    }
}
