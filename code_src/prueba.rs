fn main() {
    let mut value: i32 = 42;

    // Creamos un puntero a `value`
    let ptr: *mut i32 = &mut value;

    // Usamos código `unsafe` para desreferenciar el puntero
    unsafe {
        *ptr = 0;
        println!("El valor del puntero es: {}", *ptr);
    }

    // Imprimimos el valor original para verificar que ha cambiado
    println!("El valor original ahora es: {}", value);

    // Creamos otro puntero a `value`
    let ptr2: *mut i32 = &mut value;

    // Usamos código `unsafe` para desreferenciar el segundo puntero
    unsafe {
        *ptr2 = 10;
        println!("El valor del segundo puntero es: {}", *ptr2);
    }

    // Imprimimos el valor original para verificar que ha cambiado
    println!("El valor original ahora es: {}", value);

    // Creamos un tercer puntero a `value`
    let ptr3: *mut i32 = &mut value;

    // Usamos código `unsafe` para desreferenciar el tercer puntero
    unsafe {
        *ptr3 = 20;
        println!("El valor del tercer puntero es: {}", *ptr3);
    }

    // Imprimimos el valor original para verificar que ha cambiado
    println!("El valor original ahora es: {}", value);
}