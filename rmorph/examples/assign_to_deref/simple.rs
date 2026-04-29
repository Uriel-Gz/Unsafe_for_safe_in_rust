// Pattern: assign_to_deref (*p = value)
fn main() {
    let mut x = 0i32;
    let p = &mut x as *mut i32;
    unsafe {
        *p = 100; // asignación a la desreferencia -> `assign_to_deref`
        println!("assign_to_deref simple -> {}", *p);
    }
}
