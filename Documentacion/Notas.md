
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


## Casos de uso de codigo unsafe
[22/12/2024] empezado de analisis de codigo

[21/04/2025] Reordenacion de ideas, analisis de morfologia del codigo.

- caso 1:

        let <variable> = <string>.add(<elemento>);
        ptr::write(<variable>, <valor>);

- caso 2:

        let <variable> = <string>.add(<elemento>);
        Some(ptr::read(<variable>))

- caso 2:

        let <variable> = <string>.add(<elemento>);
        Some(&*<variable>)

- caso 3:

        let <elem> = Box::from_raw(<otroelem>);
        <otroelem> = <elem>.<next>;

- caso 4:

        (*<string>).<string> = <id>;

- caso 5:

        let <variable>: &mut <struct> = &mut *<id>;

- caso 6:

        let <variable>: *mut <struct> = <string>;

- caso 7:

        let <variable>: *mut <struct> = (*<nombre>).<id>;

- caso 8:

        (*<nombre>).<attr> = Self::<funcion>((*<nombre>).<attr>);

- caso 9:

        

- caso 10:

        
        
- caso 11:

        
        
- caso 12:

        
        
- caso 13:

        
        
- caso 14:

        
        
- caso 15:

        
        
- caso 16:

        
        
- caso 16:
        


- caso 17:



- caso 18:

        

- caso 19:



- caso 20:


