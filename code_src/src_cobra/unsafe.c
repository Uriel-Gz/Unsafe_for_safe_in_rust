/* ------------------------------------------------------------
 *  Detector de bloques `unsafe` en código fuente.
 *  Inspirado en el fragmento que analizaba `if` / `else`.
 *  Busca la sintaxis típica de Rust:
 *
 *      unsafe { … }
 *
 *  Cuando encuentra una apertura de `unsafe` sin su llave de cierre
 *  correspondiente, informa del error (similar a la función `bad`).
 * ------------------------------------------------------------ */

#include "c_api.h"

/* Estructura mínima que el motor original usaba para representar
 * cada token del flujo de entrada.  Sólo guardamos lo necesario. */
typedef struct Prim {
    const char *txt;   /* texto del token (p.e. "unsafe", "{", "}" …) */
    const char *fnm;   /* nombre del archivo (para el mensaje)   */
    int   lnr;        /* número de línea del token               */
    int   col;        /* columna de inicio (opcional)            */
    struct Prim *jmp;  /* puntero usado por el motor original para
                         saltar a la posición de “match”        */
    int   curly;      /* nivel de llaves cuando se encontró el token */
    struct Prim *next;/* siguiente token en la lista               */
} Prim;

/* Mensaje de error cuando falta la llave de cierre del bloque unsafe */
static void bad_unsafe(Prim *ptr)
{
    printf("%s:%d: missing '}' for unsafe block opened at line %d\n",
           ptr->fnm, ptr->lnr, ptr->lnr);
}

/* Macro para avanzar al siguiente token (el motor original usaba NEXT) */
#define NEXT   cur = cur->next

/* Macro para buscar el siguiente token cuyo texto sea S */
#define FIND(s)                               \
    while (cur && strcmp(cur->txt, (s)) != 0) \
        NEXT;

/* ------------------------------------------------------------
 *  cobra_main – recorre la lista de tokens y verifica que cada
 *  `unsafe` se cierre correctamente con una `}`.
 * ------------------------------------------------------------ */
void cobra_main(void)
{
    Prim *cur, *ptr;

    /* cur debe estar inicializado por el motor que alimenta los tokens */
    for ( ; cur; NEXT )
    {
        /* 1. Buscar la palabra clave "unsafe" */
        FIND("unsafe");
        /* Guardamos la posición del token "unsafe" para poder
         * reportar errores si falta la llave de cierre. */
        ptr = cur;          /* ptr apunta al token "unsafe" */
        NEXT;               /* avanzar al token siguiente */

        /* 2. El token que debe seguir es una llave de apertura "{" */
        if (cur == NULL) break;          /* fin inesperado */
        if (strcmp(cur->txt, "{") != 0) {
            /* No hay `{` justo después de `unsafe` → error de sintaxis */
            bad_unsafe(ptr);
            continue;
        }

        /* 3. Guardamos el nivel de llaves al entrar al bloque */
        int start_level = cur->curly;     /* nivel de `{` del bloque unsafe */

        /* 4. Avanzar dentro del bloque hasta encontrar la llave de cierre
         * cur->curly* disminuye cuando se encuentra `}`. */
        for ( ;; )
        {
            NEXT;
            if (cur == NULL) break;      /* fin de archivo sin cerrar */

            /* Cuando el nivel de llaves vuelve al valor anterior,
             * hemos encontrado la `}` que cierra el bloque unsafe. */
            if (cur->curly == start_level - 1)   /* una llave menos */
                break;
        }

        /* 5. Si salimos del bucle porque cur es NULL, falta la `}` */
        if (cur == NULL) {
            bad_unsafe(ptr);
            break;
        }

        /* En este punto el bloque unsafe está bien cerrado; el bucle
         * externo continuará a partir de la `}` encontrada. */
    }
}
