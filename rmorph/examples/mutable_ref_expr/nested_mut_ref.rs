// Pattern: mutable_ref_expr (referencia mutable sobre expresión paréntesis)
fn main() {
    let mut x = 99i32;
    let p = &mut x as *mut i32;
    unsafe {
        let r = &mut (*p); // referencia mutable a través de puntero crudo
        *r = 0;
        println!("mutable_ref_expr nested -> {}", x);
    }
}
