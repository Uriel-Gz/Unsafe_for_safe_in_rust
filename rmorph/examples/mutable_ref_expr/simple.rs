// Pattern: mutable_ref_expr (referencia mutable &mut dentro de unsafe)
fn main() {
    let mut x = 3i32;
    unsafe {
        let r = &mut x; // debería ser detectado como `mutable_ref_expr` dentro de unsafe
        *r = 10;
        println!("mutable_ref_expr simple -> {}", x);
    }
}
