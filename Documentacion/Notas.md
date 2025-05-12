
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
- RustInterceptor-master
- leetCode-in-rust-main
- rust-postgres-master
- cs561-rust-main
- waveterm-main
- hyper-master
- tokio-master
- sar-rs-main
- rust-master
- exa-master
- reth-main
- fd-master
- warp-main


## Cronologia
[22/12/2024] empezado de analisis de codigo

[21/04/2025] Reordenacion de ideas, analisis de morfologia del codigo. Se inicia la lista de casos de uso.

[26/04/2025] Se continua con el analisis de codigo, a partir del enfoque de solo bsucar casos de punteros raw, la busqueda es mas rapida.


## Casos de uso de codigo unsafe

### Asiganciones

- caso 1:

        let <variable> = <string>.add(<elemento>);
        ptr::write(<variable>, <valor>);

- caso 2:

        let <variable> = <string>.add(<elemento>);
        Some(ptr::read(<variable>))

- caso 3:
        -- obviar el match
        let <variable> = <string>.add(<elemento>);
        Some(&*<variable>)

- caso 4:

        let <elem> = Box::from_raw(<otroelem>);
        <otroelem> = <elem>.<next>;

- caso 5:

        (*<string>).<string> = <id>;

- caso 6:

        let <variable>: &mut <struct> = &mut *<id>;

- caso 7:

        let <variable>: *mut <struct> = <exp>;

- caso 8:

        let <variable>: *mut <struct> = (*<nombre>).<id>;

- caso 9:

        (*<nombre>).<attr> = Self::<funcion>((*<nombre>).<attr>);

- caso 10: (maybe)

        *<string> += "<string>"  

- caso 11:

        let <name> = <string>();
        (&<name>.<method>, &mut <name>.<attr>, &<name>.<string>)
        
- caso 12: (maybe)

        (*<string>).<attr> = Some(<string>);
        
- caso 13:

        *<name> = Some(<string>)
        
- caso 14:

        let <var> = *<val>
        
- caso 15:

        
        
- caso 16:

        
        
- caso 17:

        
        
- caso 18:
        
