// Pattern: raw_addr_expr (&raw mut ...)
fn main() {
    let mut v = 5i32;
    let r = &raw mut v;
    unsafe {
        // escribir a través de r no es seguro sin comprobar aliasing
        *r = 6;
        println!("raw_addr_expr mut -> {}", v);
    }
}
