# Gramáticas genéricas estilo Fandango Fuzzer para patrones Unsafe

# 1. Asignaciones Unsafe Directas (ptr::write, ptr::read, Box::from_raw)

stmt =
    "let" <var> "=" <expr> ("." <method>)? "(" <arg>? ")" ";"
    ("ptr::write" "(" <var> "," <valor> ")" ";")?
    | "let" <var> "=" <expr> ("." <method>)? "(" <arg>? ")" ";"
      "Some" "(" "ptr::read" "(" <var> ")" ")" ";"
    | "let" <var> "=" <expr> ("." <method>)? "(" <arg>? ")" ";"
      "Some" "(" "&*" <var> ")" ";"
    | "let" <raw_var> ":" "*" "mut" <tipo> "=" "Box::into_raw" "(" <var> ")" ";"
      "Ok" "(" "Box::from_raw" "(" <raw_var> "as" "*" "mut" <tipo> ")" ")" ")"

# 2. Pasar referencia y manipular punteros (*, &mut, unsafe block)

stmt =
    "*" "(" <var> ")" "." <campo> "=" <expr> ";"
    | "let" <var> ":" "&mut" <tipo> "=" "&mut" "*" <var> ";"
    | "let" <var> ":" "*" "mut" <tipo> "=" <expr> ";"
    | "let" <var> ":" "*" "mut" <tipo> "=" "*" "(" <var> ")" "." <campo> ";"
    | "*" "unsafe" "{" "&mut" "*" <var> "}" ("." <campo>)* "=" <expr> ";"
    | "let" <var> "=" "&mut" "*" "(" <var> "as" "*" "mut" <tipo> ")" ";"
    | "*" "(" <var> ")" "." <campo> "=" <tipo> "::" <funcion> "(" "*" "(" <var> ")" "." <campo> ")" ";"

# 3. Manipulación de strings (*, +=)

stmt =
    "*" <var> "+=" "\"" <string_lit> "\"" ";"

# 4. Devolución de valores y asignación con Some

stmt =
    "let" <var> "=" <funcion> "(" ")" ";"
    | "(" "&" <var> "." <campo> "," "&mut" <var> "." <campo> "," "&" <var> "." <campo> ")" ";"
    | "*" "(" <var> ")" "." <campo> "=" "Some" "(" <var> ")" ";"
    | "*" <var> "=" "Some" "(" <var> ")" ";"
    | "let" <var> "=" "*" <var> ";"
    | "let" <var> ":" <tipo> "?=" "unsafe" "{" <expr> "}"
    | "let" <var> "=" "&" <var> "as" "*" "const" <tipo> ("as" "*" "const" <tipo>)? ";"
      "*" <var> ";"
    | "unsafe" "{" <expr> "}"

# 5. Llamada a funciones externas y uso de instrucciones ensamblador

stmt =
    "extern" "\"" "C" "\"" "{"
        "fn" <funcion> "(" (<param> ":" <tipo> ("," <param> ":" <tipo>)*)? ")" "->" <tipo> ";"
    "}"
    | "core::arch::x86_64::_mm_storeu_ps" "(" <ptr> ".as_mut().as_mut_ptr()" "as" "*" "mut" <tipo> "," <valor> ")" ";"
    | "*" "(" <ptr> ".as_mut().as_mut_ptr()" "as" "*" "mut" <tipo> ")" "=" <valor> ";"

# 6. Casos especiales MaybeUninit y PhantomData con unsafe block

stmt =
    "unsafe" "{"
        "let" "mut"? <var> "=" "MaybeUninit" "::" "<" <tipo> ">" "::uninit" "(" ")" ";"
        <funcion> "(" <param>? "," "std::ptr::null()" "," <var> ".as_mut_ptr" "(" ")" ")" ";"
        <var> ".assume_init" "(" ")" ";"
    "}"
    | "unsafe" "{"
        "let" <var> "=" <tipo> "::" "::data" "(" "self.ptr" ")" ";"
        <var> ".as_ref()" ("." <campo>)* ".fetch_add" "(" "1" "," <ordenamiento> ")" ";"
        <struct> "{" "ptr:" "self.ptr," "pd:" "PhantomData," "}"
    "}"

# Tokens generales:

<var> = [a-zA-Z_][a-zA-Z0-9_]*
<expr> = .*?                       # expresión genérica (no estricto)
<method> = [a-zA-Z_][a-zA-Z0-9_]*
<campo> = [a-zA-Z_][a-zA-Z0-9_]*
<funcion> = [a-zA-Z_][a-zA-Z0-9_]*
<tipo> = [A-Z][a-zA-Z0-9_:<>]*    # tipo estilo Rust
<arg> = .*?                       # argumento genérico
<string_lit> = .*?                 # literal string
<string> = <var>
<raw_var> = <var>
<nombre> = <var>
<elemnto> = <expr>

<val> = <expr>
<param> = <var>
<valor> = <expr>
<ordenamiento> = Ordering::[A-Z_]*
<ptr> = <var>






# Gramática difusa para un bloque de código simple de asignaciones aritméticas en lenguaje tipo C/JS.

# Un bloque puede tener de 1 a 3 líneas de código (difuso)
BLOCK ::= LINE {1..3}

# Línea puede ser una asignación simple o una operación con suma, resta, multiplicación o división
LINE ::= VAR '=' EXPR ';'

# Variables permitidas (difuso: elige entre varias opciones)
VAR ::= 'x' | 'y' | 'z' | 'temp' | 'result'

# Expresión puede ser un valor numérico o una operación binaria entre variables o literales
EXPR ::=
    VALUE
  | VAR
  | EXPR OP EXPR

# Operadores posibles
OP ::= '+' | '-' | '*' | '/'

# Valores numéricos enteros (ejemplo 0-10, puede ampliarse)
VALUE ::= '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '10'

# Reglas difusas (probabilísticas o rangos implícitos):
# - El número de líneas (1 a 3) es una variación difusa.
# - VAR puede ser cualquiera del conjunto dado.
# - EXPR puede anidar operaciones para dar mayor complejidad y variabilidad.
