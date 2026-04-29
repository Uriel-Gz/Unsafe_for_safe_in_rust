// Pattern: deref_expr (desreferenciación simple)
fn main() {
    let x = 42i32;
    let p = &x as *const i32;
    unsafe {
        let v = *p; // *p debe ser detectado como `deref_expr`
        println!("deref_expr simple -> {}", v);
    }
}
