
import os
import re
import fnmatch

# strings constantes para generar el html

header = """
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Document</title>
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.5.1/styles/default.min.css" />
    <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.5.1/highlight.min.js"></script>
    <script>hljs.highlightAll();</script>
    <style>
        a:hover{
            cursor: pointer;
        }
    </style>
</head>
<body>
<pre>
"""

footer = """
</pre>
    <script>
        let f = false;
        function cargarArchivo(archivo, id, line) {
            if (!f) {
                f = true;
                fetch(archivo)
                    .then(response => {
                        if (!response.ok) {
                            throw new Error('Error al cargar el archivo');
                        }
                        return response.text();
                    })
                    .then(data => {
                        const bloque = document.getElementById(id);
                        bloque.innerText = data;
                        bloque.style.display = 'block'; // Mostrar el contenido
                        bloque.scrollTop = parseInt(line, 10) * 15; // Mostrar el contenido
                    })
                    .catch(error => {
                        console.error('Error:', error);
                    });
                }else{
                    f = false;
                    const bloque = document.getElementById(id);
                    bloque.style.display = 'none'; // Mostrar el contenido
                }
            }
    </script>
</body>
</html>
"""


def indentify_unsafe_code_in_directory(directory):
    # simple unsafe patter block
    pattern = r'unsafe\s*\{(?:[^{}]|\{(?:[^{}]|\{[^{}]*\})*\})*\}'

    file = open('unsafe_code.html', 'w')
    file.write(header)
    # last_rep = "/"

    # Recorre el directorio y sus subdirectorios
    for root, dirs, files in os.walk(directory):
        # repository = str(root).split("\\")[1]
        # if str(root).split("\\")[1:] != last_rep :
        #     file = open(f'{str(root).split("\\")[1:][:1]}.html', 'w')
        #     file.write(header)
        #     last_rep = str(root).split("\\")[1:]

        for filename in fnmatch.filter(files, '*.rs'):
            file_path = os.path.join(root, filename)

            # Lee el contenido del archivo
            with open(file_path, 'r', encoding='utf-8') as f:
                code = f.read()

            matches = re.findall(pattern, code)

            i = 1
            lines_unsafe = []
            for line in code.splitlines():
                if line.find('unsafe {') != -1:
                    lines_unsafe.append(i)
                i += 1

            pathToFile = '/'.join(str(root).split("/")[2:])
            for j in range(len(matches)):
                
                unsafe_id = f'{pathToFile}/{filename}-{lines_unsafe[j]}'
                pre_content = f'<H3>In the repository (subfolder/s) {pathToFile} </H3>\n' \
                              f'    In the file : <a onclick=\"cargarArchivo(\'{root}/{filename}\',\'{unsafe_id}\',\'{lines_unsafe[j]}\')\"><em>{filename}</em></a>\n'\
                              f'<code class="rust" style="border-radius: 10px;">\n'
                pos_content = f'</code>\n' \
                              f'<code class=\"rust\" id=\"{unsafe_id}\" style=\"overflow: auto; height: 150px; display: none;\"></code>\n'
                file.write(pre_content)
                file.write(f'{matches[j]}\n')
                file.write(pos_content)
    
    file.write(footer)


# Llama a la función con la ruta del directorio
indentify_unsafe_code_in_directory('../projects_to_review')


