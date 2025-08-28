
# posibles reemplazos

<replace> = <direct_asign> | <miss_match>

# asignacion directa

<direct_asign> = "replace(&" <string> "," <string> ");"

# desreferecniacion por matchs

<miss_match> = "match(" <string> ") {" <string_lit> "}"

# asignacion de referencias

<reference_asign> = None

# asignacion de bloques unsafe

<unsafe_asign> = "Box::from_raw(" <string_list> ")"

# asignacion de bloques unsafe puunteros

<ptr_unsafe_asign> = "Box::from_raw(" <string> ")" <string> ";"

# transformacion de casteo a metodo

<method_mut_cast> = <string>".as_mut_ptr()"


# otros casos

<others> = <string_list>






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