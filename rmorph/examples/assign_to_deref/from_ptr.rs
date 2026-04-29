// Pattern: assign_to_deref (asignar leyendo desde otro puntero)
fn main() {
    let mut a = 1i32;
    let b = 2i32;
    let pa = &mut a as *mut i32;
    let pb = &b as *const i32;
    unsafe {
        *pa = *pb; // asignación desde otra desreferencia
        println!("assign_to_deref from_ptr -> {}", *pa);
    }
}
