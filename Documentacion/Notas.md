
<!-- 
<> Observaciones del analisis del código
<>-->

# Trabajo Final (Tesis)

## Observación de cantidad de bloques unsafe

 [21/11/2024] Al ejecutar la herramienta de identificacion de codigo unsafe sobre 6 repositorios extraidos de github, la cantidad de bloques con dicha caracteristicas registrados fue de 634, lo cual deja en evidencia su uso frecuente para diversas cuestiones.


## Corrector de código unsafe

[21/11/2024] El corrector esta en progreso de poder cambiar el codigo inseguro por uno que este verificado, de momento solo cambia por expresiones asignadas por el usuario.

## Librerias encontradas que trabajan con unsafe
1. ffi
2. mem
3. libc::fcntl
4. std::os::unix::io::RawFd
5. std::os::unix::io::RawHandle
6. Pin
7. mio::net::UnixListener::from_raw_fd
8. mio::net::UnixStream::from_raw_fd
9. mio::net::UnixDatagram::from_raw_fd
10. BrorrowedFd::brorrow_raw


## Repositorios analisados
- rust-postgres-master
- mnemos-alloc-main
- stm32f042-master
- tokio-master
- hyper-master
- gxhash-main
- lucet-main
- exa-master
- reth-main
- warp-main
- fd-master

- rust-master           // falta verificar
- rure-master           // no aporta por ser casos triviales

### Sin codigo unsafe
- RustInterceptor-master
- LeetCode-in-rust-main
- cs561-rust-main
- amaber-staging
- waveterm-main
- sat-rs-main


## Cronologia
[22/12/2024] empezado de analisis de codigo, sin progreso, la idea fue cambiando mientras mas veia el codigo.

[21/04/2025] Reordenacion de ideas, analisis de morfologia del codigo. Se inicia la lista de casos de uso. Extraccion de codigo en archivos .txt (forma mas rapida, leer y escribir).

[26/04/2025] Se continua con el analisis de codigo, a partir del enfoque de solo buscar casos de punteros raw, la busqueda es mas rapida. Veo la idea de desreferenciacion.

[12/05/2025] Cambio en extracion de codigo unsafe, busqueda mas inteligente y comienzo de clasificacion. Primer tipo: desreferecniacion de punteros (asignacion, devolucion, etc.).

[17/05/2025] Reanalisis de codigo en los repositorios, repasando donde y como se usan los elementos de bloque unsafe. Tomo nota de los repositorios para despues llevar mas profundo el analisis, ademas intento categorizar el tipo de contenido que se omite o que se puede usar.

[19/05/2025] Intento extraer codigo de rust-master para analisar. El inteto con exito reducido, al ser el repositorio oficial de rust la distribucion y la cantidad de codigo que maneja le dificulta al programa terminar con su contenido, no obstante se obtienen una gran cantidad de casos diferentes, el analisis de estos es mas puntual y de mayor complejidad, pero sera importante (quiero creer) de alguna manera.

[20/05/2025] Añado anotaciones (comentarios) al codigo para que sea mas legible, elimino los .txt generados en la primera etapa de analisis, deduzco que el archivo identifyer no es necesario, dado que es una reduccion del extractor.
- Modifico archivo modifier.py para guardar una lista de patrones, los cuales podran ser aplicados en implementaciones posteriores  y destino una carpeta ``result`` para los archivos modificados.
- Agrego archivo ``Detection_and_replace_of_unsafe_code.doc``.

[09/06/2025] Trato de crear una breve gramatica para las posibles modificaciones que se pueden hacer a partir de los casos de codigo vistos, veo la descripcion de funciones en las bibliotecas de rust, distingo algunas funciones de mem y algunos casos vistos de forma puntual.

[10/06/2025] Continuo con el analisis de funciones de biblioteca.

[18/08/2025] Traspaso los casos de posible equivalencia desde las bibliotecas estandares hacia una tabla en la documentacion usando como referencia los casos extraidos de los repositorios analisados.

[25/08/2025] - [29/08/2025] Escribimos un poco del proceso y los datos que se van recolectando en el informe final.

[08/10/2025] - [11/10/2025] Voy implementando la idea del programa final, partindo de definir algunos patros ejemplos, y sus posibles reemplazos, aplico en el archivo de prueba.rs.
ideas:
        - extractor deberia generar un archivo con los archivos a modificar
        - modifier deberia tomar el archivo con los cambios y usarlo para modificar solo eso sin tener que ver todos los demas (S/N)

[20/10/2025] Hoy cambio el enfoque para tratar los patrones, ahora en lugar de una lista con patrones y otra con posibles reemplazos, trabajo cada patron como si fuera una unidad independiente, conteniendo el mismo ambas cosas y modelandolos a partir de una pequeña interface IPattern.

[22/10/2025] Empiezo a cambiar el esquema de gramatica para que coincida con la idea de tener cada tipo de unsafe su propio conjunto de patrones y posibilidad de reemplar.

[24/10/2025] El uso de gramatica en lugar de expresiones regulares va tomando forma, pude hacer que tome los parametros que detecto y los sustituya en el codigo, ademas arregle el detalle de saltearme lineas.

[06/11/2025] - [11/11/2025] Profundizo en una descripcion detallada de la gramatica general que deberia usar la herramienta, ajusto los casos de u_desreference y agrego algunos casos para analizar en el archivo de pruebas.

[14/11/2025] Plasmo las ideas de lo que he estado haciendo, tratare de resumir todo el funionamiento para ser consiso.

[25/11/2025] - [30/11/2025] Genero una version en rust del extractor, con algunas modificaciones y extras.

[06/12/2025] Comienzo un analisis de la estructura del ast de rust con la finalidad e poder usarlo para realizar los reemplazos de codigo.

[05/01/2026] Pasos para etapa de reemplazo:

- generar todas las agrupaciones de tipos de codigo unsafe 

- investigar como y donde puedo aplicar los cambios que se pueden detectar atraves del extractor

- completar el archivo modifier




## Ventajas sobre `modifier.py`

| Aspecto | Python | Rust |
|---------|--------|------|
| Parseador | Token-based | AST-based (syn) |
| Type Safety | No | Sí |
| Performance | Interpretado | Compilado nativo |
| Detección de patrones | Manual | Automática |
| Templates | No | Dinámicos con placeholders |
| Reportes | No | JSON + estadísticas |
| Extensibilidad | Difícil | Fácil (agregar templates) |



## 🔄 Flujo de Datos

```
┌─────────────────────────────────────┐
│ Directorio de entrada (archivos .rs) │
└────────┬────────────────────────────┘
         │
         ↓
┌────────────────────────────────────┐
│ replace_unsafe_code() [MAIN]        │
│                                     │
│ ┌──────────────────────────────┐   │
│ │ Para cada archivo .rs:       │   │
│ │                              │   │
│ │ 1. Leer contenido            │   │
│ │ 2. Parsear → AST             │   │
│ │ 3. Detectar patrones         │   │
│ │ 4. Aplicar transformaciones  │   │
│ │ 5. Formatear                 │   │
│ │ 6. Escribir resultado        │   │
│ │                              │   │
│ └──────────────────────────────┘   │
│                                     │
│ ┌──────────────────────────────┐   │
│ │ Si patterns_dir.is_some():   │   │
│ │ → Generar summary.json       │   │
│ └──────────────────────────────┘   │
└────────┬────────────────────────────┘
         │
         ├──→ Directorio de salida (código transformado)
         └──→ patterns/summary.json (estadísticas, opcional)
```




## Casos de uso de codigo unsafe

### Asiganciones

#### obviar el match

- caso 1:  structures/src/stack.rs

        let <variable> = <string>.add(<elemento>);
        ptr::write(<variable>, <valor>);

- caso 2: structures/src/queue.rs           (similar)
          structures/src/stack.rs

        let <variable> = <string>.add(<elemento>);
        Some(ptr::read(<variable>))

- caso 3: hyper-master/src/rt/timer.rs
          structures/src/stack.rs
        
        let <variable> = <string>.add(<elemento>);
        Some(&*<variable>)

<!-- tengo que generalizar -->
- caso 4: structures/src/avl.rs

        let <elem> = Box::from_raw(<otroelem>);
        <otroelem> = <elem>.<next>;


#### pasar referencia

- caso 5: caso generico de uso, strutures.rs

        (*<string>).<string> = <id>;

- caso 6: structures/src/avl.rs

        let <variable>: &mut <struct> = &mut *<id>;

- caso 7: structures/src/avl.rs

        let <variable>: *mut <struct> = <exp>;

- caso 8:  structures/src/avl.rs

        let <variable>: *mut <struct> = (*<nombre>).<id>;

- caso 9:  structures/src/avl.rs

        (*<nombre>).<attr> = Self::<funcion>((*<nombre>).<attr>);

#### manejo de strings

- caso 10: tokio-master/tokio/src/future/maybe_done.rs

        *<string> += "<string>"  

#### devolucion de valores

- caso 11: tokio-master/tokio/src/runtime/io/scheduled_io.rs

        let <name> = <string>();
        (&<name>.<method>, &mut <name>.<attr>, &<name>.<string>)
        
- caso 12: hyper-master/src/ffi/task.rs (parecido)

        (*<string>).<attr> = Some(<string>);
        
- caso 13: 

        *<name> = Some(<string>)
        
- caso 14:

        let <var> = *<val>
        
- caso 15: gxhash-main/benches/throughput_criterion.rs

        let <var> = unsafe { alloc(x)};
        
- caso 16: gxhash-main/benches/throughput_criterion.rs
           hyper-master/src/ffi/body.rs
           hyper-master/src/proto/h1/io.rs
           lucet-main/lucet-runtime/lucet-runtime-internals/src/alloc/tests.rs
           lucet-main/lucet-runtime/lucet-runtime-internals/src/alloc/tests.rs
           mnemos-alloc-main/src/heap.rs

        let <var>: <tipo>? = unsafe { <expresion> };
        
- caso 17: gxhash-main/benches/throughput_criterion.rs
           

        unsafe { dealloc(x,y)}
        
- caso 18: gxhash-main/src/hasher.rs
           gxhash-main/src/gxhash/mod.rs
        
        let <var> = &<regex> as *const <tipo> as *const <tipo>;
        *<var>

- caso 19: hyper-master/benches/support/tokiort.rs 
           hyper-master/src/common/io/compat.rs
           hyper-master/src/ffi/http_types.rs
           reth-main/crates/net/network/src/session/conn.rs

        analisar detenidamente, puede haber uso en otra parte/ tiene matchs

- caso 20: hyper-master/src/upgrade.rs

        let raw: *mut dyn Io = Box::into_raw(self);
        Ok(Box::from_raw(raw as *mut T))

- caso 21: hyper-master/src/ffi/http_types.r   
           structures/src/avl.rs

        *unsafe { &mut *req }.0.uri_mut() = u;
        
- caso 22: hyper-master/src/ffi/http_types.rs

        let vec = &mut *(vec as *mut Vec);

- caso 23: hyper-master/src/proto/h2/role.rs
        lucet-main/lucet-runtime/lucet-runtime-internals/src/instance.rs
        lucet-main/lucet-runtime/lucet-runtime-internals/src/vmctx.rs
        lucet-main/lucet-runtime/lucet-runtime-internals/src/context/mod.rs
        lucet-main/lucet-runtime/lucet-runtime-internals/src/context/tests/rust_child.rs
        lucet-main/lucet-runtime/lucet-runtime-tests/src/globals.rs

        unsafe { val };
        <!-- generico a lo que devuelva -->

- caso 24: hyper-master/src/rt/io.rs
           tokio-master/tokio/src/io/util/read_buf.rs

        unsafe { &mut *(raw as *mut [u8] as *mut [MaybeUninit]) }

- caso 25: lucet-main/benchmarks/lucet-benchmarks/src/context.rs
           lucet-main/lucet-runtime/lucet-runtime-internals/src/context/tests/mod.rs

        let mut <var> = <regex>;

#### variaciones de unsafe

- caso 26: lucet-main/lucet-concurrency-tests/src/killswitch.rs
           lucet-main/lucet-runtime/lucet-runtime-internals/src/alloc/tests.rs
           lucet-main/lucet-runtime/lucet-runtime-internals/src/context/tests/c_child.rs
           reth-main/crates/cli/util/src/sigsegv_handler.rs
        
        llamada a funcion externa en c

- caso 27: lucet-main/lucet-concurrency-tests/src/killswitch.rs
           lucet-main/lucet-runtime/lucet-runtime-internals/src/context/tests/rust_child.rs

        <identificador> = some(<x>);

- caso 28: lucet-main/lucet-runtime/lucet-runtime-internals/src/c_api.rs

        casos de pasaje de valores a su representacion

- caso 29: mismo repo de arriba

        core::arch::x86_64::_mm_storeu_ps(
                    v.fp.as_mut().as_mut_ptr() as *mut f32,
                    retval.fp(),
                );
        *(v.gp.as_mut().as_mut_ptr() as *mut u64) = retval.gp();

- caso 30: lucet-main/lucet-runtime/lucet-runtime-internals/src/val.rs...

        let <var> = <method>;
        _mm_storeu_pd(&mut out[0] as *mut f64, vd);

- caso 31: lucet-main/lucet-runtime/lucet-runtime-internals/src/alloc/tests.rs
           lucet-main/lucet-runtime/lucet-runtime-internals/src/context/tests/rust_child.rs
            lucet-main/lucet-runtime/lucet-runtime-tests/src/globals.rs

        casos con asserts

- caso 32: lucet-main/lucet-runtime/lucet-runtime-internals/src/sysdeps/freebsd.rs

        &mut unsafe { self.0.as_mut().unwrap() }.uc_mcontext;
        mcontext.mc_rip = new_ip as i64;

- caso 33: lucet-main/lucet-runtime/lucet-runtime-tests/src/guest_fault.rs

        *super::RECOVERABLE_PTR = 0;

- caso 34: lucet-main/lucet-runtime/lucet-runtime-tests/src/helpers.rs
        reth-main/crates/storage/libmdbx-rs/benches/cursor.rs
        reth-main/crates/storage/libmdbx-rs/benches/transaction.rs

        <!-- se usa como parte de una expresion -->

        unsafe {
                let mut out = MaybeUninit::::uninit();
                sigaction(*sig, std::ptr::null(), out.as_mut_ptr());
                out.assume_init()
            }

- caso 35: mnemos-alloc-main/src/containers.rs (clone)

        unsafe {
            let aitem_nn = Active::>::data(self.ptr);
            aitem_nn.as_ref().refcnt.fetch_add(1, Ordering::SeqCst);

            HeapArc {
                ptr: self.ptr,
                pd: PhantomData,
            }
        }


#### casos especiales para reveer

* mnemos-alloc-main/src/containers.rs
        
        unsafe {
            let aiptr: *mut ArcInner = Active::>::data(self.ptr).as_ptr();
            let dptr: *const T = addr_of!((*aiptr).data);
            &*dptr
        }
---
        unsafe {
            let (nn_ptr, count) = ActiveArr::::data(self.ptr);
            forget(self);
            (nn_ptr, count)
        }

*  tokio-master/tokio/src/util/idle_notified_set.rs
        
        unsafe {
                let old_my_list = *ptr;
                *ptr = List::Neither;
                old_my_list
            }





<> en mnemo-alloc hay mucha deref and drop
<> en reth-main hay variedad de asserts
<> en stm32f042 hay punteros que devuelven su referencia &*, ademas se usan mucho pasaje unsafe como parametro
<> en tokio tambien pasa similar a stm y usa mucha libreria 
<> caso 21 y 24 sintaxis interna simmilar


Clasificación de casos de código unsafe

Asignaciones
Caso 1: Asignación directa (ptr::write)
Caso 2: Asignación con lectura (ptr::read)
Caso 3: Asignación con referencia (&*)
Pasar referencia
Caso 4: Pasar referencia a una estructura (&mut *)
Caso 5: Pasar referencia a un campo de una estructura ((*).)
Manejo de strings
Caso 6: Concatenación de strings (+=)
Devolución de valores
Caso 7: Devolución de un valor (return)
Caso 8: Devolución de un valor con Some
Caso 9: Devolución de un valor con *
Variaciones de unsafe
Caso 10: Llamada a función externa en C
Caso 11: Pasaje de valores a su representación
Caso 12: Uso de instrucciones de ensamblaje (_mm_storeu_ps)
Casos especiales
Caso 13: Uso de MaybeUninit y assume_init
Caso 14: Uso de PhantomData y fetch_add


Clasificación del Uso de Punteros en Rust

    Dereferenciación
        Dereferenciación Directa: Uso de * para acceder al valor apuntado por un puntero crudo (*const T o *mut T).
        Dereferenciación de Referencias: Uso de & para acceder al valor de una referencia (&T o &mut T).
        Dereferenciación Segura vs. Insegura:
            Segura: Uso de referencias y punteros que garantizan la validez del acceso (ej. &T, &mut T).
            Insegura: Uso de punteros crudos sin garantías de validez (ej. *const T, *mut T).

    Acceso a Memoria
        Acceso a Memoria Estática: Uso de punteros para acceder a datos en la memoria estática (ej. variables globales).
        Acceso a Memoria Dinámica: Uso de punteros para acceder a datos en la memoria dinámica (ej. a través de Box<T> o Vec<T>).

    Mutabilidad
        Acceso Inmutable: Uso de punteros o referencias que no permiten modificar el valor apuntado (ej. &T).
        Acceso Mutable: Uso de punteros o referencias que permiten modificar el valor apuntado (ej. &mut T, *mut T).

    Transferencia de Propiedad
        Propiedad Transferida: Uso de punteros que transfieren la propiedad de los datos (ej. Box<T>).
        Propiedad Compartida: Uso de punteros que permiten compartir la propiedad sin transferirla (ej. Rc<T>, Arc<T>).
